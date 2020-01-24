use rayon::prelude::*;
use super::math::Vec4;
use super::math::Ray;
use super::image;
use super::parameters::Parameters;
use super::parameters::DebugMode;
use super::statistics::Statistics;
use super::scene::Scene;
use super::materials;

pub struct Renderer<'a>
{
    parameters: Option<&'a Parameters>,
    scene: Option<&'a Scene>,

    thread_pool: rayon::ThreadPool,

    debug_diffuse_material: materials::Material,
    debug_normals_material: materials::Material
}

impl<'a> Default for Renderer<'a>
{
    fn default() -> Self
    {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .build().expect("Failed to create a thread pool!");

        Renderer
        {
            parameters: None,
            scene: None,

            thread_pool,

            debug_diffuse_material: materials::Diffuse::new(Vec4::new(0.5, 0.5, 0.5, 1.0)),
            debug_normals_material: materials::Normals::new()
        }
    }
}

impl<'a> Renderer<'a>
{
    pub fn new() -> Self
    {
        Renderer::default()
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

    pub fn render(&self) -> image::Surface
    {
        // Start measuring render time.
        let begin_time = std::time::Instant::now();

        // Acquire required parameters.
        let parameters = self.parameters.expect("Cannot render image without parameters!");
        let scene = self.scene.expect("Cannot render image without scene!");

        // Build camera that will be used to calculate initial raycasts.
        let camera = scene.camera.build(parameters.image_width as f32 / parameters.image_height as f32)
            .expect("Failed to build compiled camera structure!");
        
        // Calculate image constants.
        let image_width_inv = 1.0 / parameters.image_width as f32;
        let image_height_inv = 1.0 / parameters.image_height as f32;
        let image_pixel_count = parameters.image_width * parameters.image_height;
        assert!(image_pixel_count > 0, "Pixel count cannot be zero!");

        // Create antialiasing kernel.
        assert!(parameters.antialias_samples >= 1, "Antialias samples must equal one or higher!");
        let antialias_subpixel_count = parameters.antialias_samples.pow(2) as usize;
        let antialias_subpixel_step = 1.0 / f32::from(parameters.antialias_samples);

        let antialias_kernel: Vec<(f32, f32)> = 
        {
            let mut kernel = Vec::with_capacity(antialias_subpixel_count);

            for subpixel_x in 0..parameters.antialias_samples
            {
                for subpixel_y in 0..parameters.antialias_samples
                {
                    let offset_u = f32::from(subpixel_x) * antialias_subpixel_step;
                    let offset_v = f32::from(subpixel_y) * antialias_subpixel_step;

                    kernel.push((offset_u, offset_v));
                }
            }

            kernel
        };

        debug_assert!(antialias_kernel.len() == antialias_subpixel_count);

        // Render pixels in parallel and collect stats.
        let mut image_pixels: Vec<Vec4> = Vec::with_capacity(image_pixel_count);
        image_pixels.resize(image_pixel_count, Vec4::new(0.0, 0.0, 0.0, 0.0));

        /* chunk rendering
        let per_thread_chunk_size = 1; //image_pixel_count / rayon::current_num_threads();
        let accumulated_stats: Statistics = image_pixels.par_chunk_mut(per_thread_chunk_size).enumerate().map(|(chunk_index, chunk)|
        */

        let accumulated_stats: Statistics = self.thread_pool.install(||
        {
            image_pixels.par_iter_mut().enumerate().map(|(pixel_index, pixel)|
            {
                /* chunk rendering
                let mut chunk_stats = Statistics::new();
                chunk.iter_mut().enumerate().for_each(|(pixel_index, pixel)|
                {
                */

                    let mut pixel_stats = Statistics::new_pixel();

                    let x = (/* per_thread_chunk_size * chunk_index + */ pixel_index) % parameters.image_width as usize;
                    let y = (/* per_thread_chunk_size * chunk_index + */ pixel_index) / parameters.image_width as usize;

                    let mut accumulated_color = Vec4::new(0.0, 0.0, 0.0, 0.0);

                    for (offset_u, offset_v) in &antialias_kernel
                    {
                        let u = (x as f32 + offset_u) * image_width_inv as f32;
                        let v = (y as f32 + offset_v) * image_height_inv as f32;
                        
                        let sample = self.sample(camera.calculate_ray(u, v), 0, &mut pixel_stats);
                        debug_assert!(sample.is_valid());

                        accumulated_color += sample;
                    }

                    *pixel = accumulated_color / antialias_subpixel_count as f32;
                    
                    pixel_stats.subpixels += antialias_subpixel_count;

                /* chunk rendering
                    chunk_stats = chunk_stats.accumulated(&pixel_stats);
                });

                chunk_stats
                */

                pixel_stats
            }).sum()
        });

        // Perform gamma correction on color values.
        let gamma_correction = 1.0 / 2.2;

        image_pixels.par_iter_mut().for_each(|pixel|
        {
            pixel.set_r(pixel.get_r().powf(gamma_correction));
            pixel.set_g(pixel.get_g().powf(gamma_correction));
            pixel.set_b(pixel.get_b().powf(gamma_correction));
            
            debug_assert!(pixel.is_valid());
            debug_assert!((pixel.get_a() - 1.0).abs() < std::f32::EPSILON);
        });

        // Print render statistics.
        println!("Rendered image in {} seconds.", begin_time.elapsed().as_secs_f32());
        accumulated_stats.print();

        // Return image with rendered pixel data.
        image::Surface::from(parameters.image_width, parameters.image_height, image_pixels)
    }

    fn sample(&self, ray: Ray, scatter_index: u16, stats: &mut Statistics) -> Vec4
    {
        let parameters = self.parameters.expect("Cannot render image without parameters!");
        let scene = self.scene.expect("Cannot render image without scene!");

        if scatter_index > parameters.scatter_limit
        {
            return Vec4::black();
        }

        stats.samples += 1;
        
        if let Some((intersection, object)) = scene.intersect(&ray, 0.0001, std::f32::MAX)
        {
            stats.intersections += 1;
            
            let material = match parameters.debug_mode
            {
                None => object.get_material(),
                Some(DebugMode::Diffuse) => &self.debug_diffuse_material,
                Some(DebugMode::Normals) => &self.debug_normals_material
            };
            
            let (scattered_ray, attenuation) = material.scatter(&ray, &intersection, scatter_index);

            if let Some(scattered_ray) = scattered_ray
            {
                stats.scatters += 1;
                stats.max_scatters = std::cmp::max(stats.max_scatters, scatter_index);

                self.sample(scattered_ray, scatter_index + 1, stats) * attenuation
            }
            else
            {
                attenuation
            }
        }
        else
        {
            match parameters.debug_mode
            {
                None =>
                {
                    let alpha = (ray.direction().get_z() + 1.0) * 0.5;
                    Vec4::new(1.0, 1.0, 1.0, 1.0) * (1.0 - alpha) + Vec4::new(0.5, 0.7, 1.0, 1.0) * alpha
                },
                Some(DebugMode::Diffuse) => Vec4::new(0.5, 0.5, 0.5, 1.0),
                Some(DebugMode::Normals) => Vec4::new(0.5, 0.0, 0.5, 1.0)
            }
        }
    }
}
