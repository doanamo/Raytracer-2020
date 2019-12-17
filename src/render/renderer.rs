use rayon::prelude::*;
use crate::math::*;
use crate::image::Image;
use super::parameters::Parameters;
use super::parameters::DebugMode;
use super::stats::RenderStats;
use super::scene::Scene;
use super::material;

pub struct Renderer<'a>
{
    parameters: Option<&'a Parameters>,
    scene: Option<&'a Scene>,

    debug_diffuse_material: material::Diffuse,
    debug_normals_material: material::Normals
}

impl<'a> Renderer<'a>
{
    pub fn new() -> Self
    {
        Renderer
        {
            parameters: None,
            scene: None,

            debug_diffuse_material: material::Diffuse::from(Color::new(0.5, 0.5, 0.5, 1.0)),
            debug_normals_material: material::Normals::new()
        }
    }

    pub fn set_parameters(mut self, parameters: &'a Parameters) -> Self
    {
        self.parameters = Some(parameters);
        self
    }

    pub fn set_scene(mut self, scene: &'a Scene) -> Self
    {
        self.scene = Some(scene);
        self
    }

    pub fn render(&self) -> Image
    {
        // Start measuring render time.
        let begin_time = std::time::Instant::now();

        // Acquire required parameters.
        let parameters = self.parameters.expect("Cannot render image without parameters!");
        let scene = self.scene.expect("Cannot render image without scene!");

        // Build camera that will be used to calculate initial raycasts.
        let camera = scene.camera.build(parameters.image_width as f32 / parameters.image_height as f32);
        
        // Calculate image constants.
        let image_width_inv = 1.0 / parameters.image_width as f32;
        let image_height_inv = 1.0 / parameters.image_height as f32;
        let image_pixel_count = parameters.image_width * parameters.image_height;
        assert!(image_pixel_count > 0, "Pixel count cannot be zero!");

        // Create antialiasing kernel.
        assert!(parameters.antialias_samples >= 1, "Antialias samples must equal one or higher!");
        let antialias_subpixel_count = parameters.antialias_samples.pow(2) as usize;
        let antialias_subpixel_step = 1.0 / parameters.antialias_samples as f32;

        let antialias_kernel: Vec<(f32, f32)> = 
        {
            let mut kernel = Vec::with_capacity(antialias_subpixel_count);

            for subpixel_x in 0..parameters.antialias_samples
            {
                for subpixel_y in 0..parameters.antialias_samples
                {
                    let offset_u = subpixel_x as f32 * antialias_subpixel_step;
                    let offset_v = subpixel_y as f32 * antialias_subpixel_step;

                    kernel.push((offset_u, offset_v));
                }
            }

            kernel
        };

        debug_assert!(antialias_kernel.len() == antialias_subpixel_count);

        // Render pixels in parallel and collect stats.
        let mut image_pixels: Vec<Color> = Vec::with_capacity(image_pixel_count);
        image_pixels.resize(image_pixel_count, Color::new(0.0, 0.0, 0.0, 0.0));

        let stats_sum: RenderStats = image_pixels.par_iter_mut().enumerate().map(|(index, pixel)|
        {
            let mut stats = RenderStats::new_pixel();

            let x = index % parameters.image_width as usize;
            let y = index / parameters.image_width as usize;

            let mut accumulated_color = Color::new(0.0, 0.0, 0.0, 0.0);

            for (offset_u, offset_v) in antialias_kernel.iter()
            {
                let u = (x as f32 + offset_u) * image_width_inv as f32;
                let v = (y as f32 + offset_v) * image_height_inv as f32;
                
                let sample = self.sample(camera.calculate_ray(u, v), 0, &mut stats);
                debug_assert!(sample.is_valid());

                accumulated_color += sample;
            }

            *pixel = accumulated_color / antialias_subpixel_count as f32;
            
            stats.subpixels += antialias_subpixel_count;
            stats
        }).sum();

        // Perform gamma correction on color values.
        let gamma_correction = 1.0 / 2.2;

        image_pixels.par_iter_mut().for_each(|pixel|
        {
            pixel.r = pixel.r.powf(gamma_correction);
            pixel.g = pixel.g.powf(gamma_correction);
            pixel.b = pixel.b.powf(gamma_correction);
            debug_assert!(pixel.is_valid());
            debug_assert!(pixel.a == 1.0);
        });

        // Print render statistics.
        println!("Rendered image in {} seconds.", begin_time.elapsed().as_secs_f32());
        stats_sum.print();

        // Return image with rendered pixel data.
        Image::from(parameters.image_width, parameters.image_height, image_pixels)
    }

    fn sample(&self, ray: Ray, scatter_index: u16, stats: &mut RenderStats) -> Color
    {
        let parameters = self.parameters.expect("Cannot render image without parameters!");
        let scene = self.scene.expect("Cannot render image without scene!");

        if scatter_index > parameters.scatter_limit
        {
            return Color::black();
        }

        stats.samples += 1;
        
        if let Some(intersection) = scene.intersect(&ray, 0.0001, std::f32::MAX)
        {
            stats.intersections += 1;
            
            let material = match parameters.debug_mode
            {
                None => intersection.material,
                Some(DebugMode::Diffuse) => &self.debug_diffuse_material,
                Some(DebugMode::Normals) => &self.debug_normals_material,
            };
            
            let (scattered_ray, attenuation) = material.scatter(&ray, &intersection, scatter_index);

            if let Some(scattered_ray) = scattered_ray
            {
                stats.scatters += 1;
                stats.max_scatters = std::cmp::max(stats.max_scatters, scatter_index);

                return self.sample(scattered_ray, scatter_index + 1, stats) * attenuation;
            }
            else
            {
                return attenuation;
            }
        }
        else
        {
            match parameters.debug_mode
            {
                Some(DebugMode::Diffuse) => return Color::new(0.5, 0.5, 0.5, 1.0),
                Some(DebugMode::Normals) => return Color::new(0.5, 0.0, 0.5, 1.0),
                None =>
                {
                    let alpha = (ray.get_direction().z + 1.0) * 0.5;
                    return Color::new(1.0, 1.0, 1.0, 1.0).mul_rgb(1.0 - alpha).add_rgb(Color::new(0.5, 0.7, 1.0, 1.0).mul_rgb(alpha));
                }
            };
        }
    }
}
