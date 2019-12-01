use serde::{ Serialize, Deserialize };
use crate::math::*;
use crate::image::Image;
use super::camera::Camera;
use super::scene::Scene;
use super::material;
use super::debug;
use super::debug::DebugMode;

#[derive(Serialize, Deserialize)]
pub struct Parameters
{
    pub image_width: usize,
    pub image_height: usize,
    pub antialias_samples: u16,
    pub scatter_limit: u16,
    pub debug_mode: Option<DebugMode>
}

impl Default for Parameters
{
    fn default() -> Self
    {
        Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 4,
            scatter_limit: 8,
            debug_mode: None
        }
    }
}

pub struct Renderer<'a>
{
    parameters: Option<&'a Parameters>,
    camera: Option<&'a Camera>,
    scene: Option<&'a Scene>,

    debug_diffuse_material: material::Diffuse,
    debug_normals_material: debug::VisualizeNormals,
    stats: debug::RenderStats
}

impl<'a> Renderer<'a>
{
    pub fn new() -> Self
    {
        Renderer
        {
            parameters: None,
            camera: None,
            scene: None,

            debug_diffuse_material: material::Diffuse::from(Color::new(0.5, 0.5, 0.5, 1.0)),
            debug_normals_material: debug::VisualizeNormals::new(),
            stats: debug::RenderStats::new()
        }
    }

    pub fn set_parameters(mut self, parameters: &'a Parameters) -> Self
    {
        self.parameters = Some(parameters);
        self
    }

    pub fn set_camera(mut self, camera: &'a Camera) -> Self
    {
        self.camera = Some(camera);
        self
    }

    pub fn set_scene(mut self, scene: &'a Scene) -> Self
    {
        self.scene = Some(scene);
        self
    }

    pub fn render(mut self) -> Image
    {
        let parameters = self.parameters.expect("Cannot render image without parameters!");
        let camera = self.camera.expect("Cannot render image without camera!");

        let begin_time = std::time::Instant::now();

        debug_assert!(parameters.antialias_samples > 0);
        let antialias_subpixel_step = 1.0 / parameters.antialias_samples as f32;
        
        let mut image = Image::new(parameters.image_width, parameters.image_height);

        for y in 0..image.get_height()
        {
            for x in 0..image.get_width()
            {
                self.stats.pixels += 1;

                let mut color = Color::new(0.0, 0.0, 0.0, 0.0);

                for subpixel_x in 0..parameters.antialias_samples
                {
                    for subpixel_y in 0..parameters.antialias_samples
                    {
                        self.stats.subpixels += 1;

                        let offset_u = subpixel_x as f32 * antialias_subpixel_step;
                        let offset_v = subpixel_y as f32 * antialias_subpixel_step;

                        let u = (x as f32 + offset_u) / image.get_width() as f32;
                        let v = (y as f32 + offset_v) / image.get_height() as f32;
                        
                        let sample = self.sample(camera.calculate_ray(u, v), 0);
                        debug_assert!(sample.is_valid());

                        color += sample;
                    }
                }

                color /= parameters.antialias_samples.pow(2);

                let gamma = 1.0 / 2.2;
                color.r = color.r.powf(gamma);
                color.g = color.g.powf(gamma);
                color.b = color.b.powf(gamma);
                debug_assert!(color.is_valid());
                debug_assert!(color.a == 1.0);

                image.set_pixel(x, y, color);
            }
        }

        println!("Rendered image in {} seconds.", begin_time.elapsed().as_secs_f32());
        self.stats.print();

        image
    }

    fn sample(&mut self, ray: Ray, scatter_index: u16) -> Color
    {
        let parameters = self.parameters.expect("Cannot render image without parameters!");
        let scene = self.scene.expect("Cannot render image without scene!");

        if scatter_index > parameters.scatter_limit
        {
            return Color::black();
        }

        self.stats.samples += 1;
        
        if let Some(intersection) = scene.intersect(&ray, 0.0001, std::f32::MAX)
        {
            self.stats.intersections += 1;
            
            let material = match parameters.debug_mode
            {
                None => intersection.material,
                Some(DebugMode::Diffuse) => &self.debug_diffuse_material,
                Some(DebugMode::Normals) => &self.debug_normals_material,
            };
            
            let (scattered_ray, attenuation) = material.scatter(&ray, &intersection, scatter_index);

            if let Some(scattered_ray) = scattered_ray
            {
                self.stats.scatters += 1;
                self.stats.max_scatters = std::cmp::max(self.stats.max_scatters, scatter_index);

                return self.sample(scattered_ray, scatter_index + 1) * attenuation;
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
                Some(DebugMode::Normals) => return Color::new(0.5, 0.5, 1.0, 1.0),
                None =>
                {
                    let alpha = (ray.get_direction().z + 1.0) * 0.5;
                    return Color::new(1.0, 1.0, 1.0, 1.0).mul_rgb(1.0 - alpha).add_rgb(Color::new(0.5, 0.7, 1.0, 1.0).mul_rgb(alpha));
                }
            };
        }
    }
}
