use crate::math::*;
use crate::image::Image;
use super::camera::Camera;
use super::scene::Scene;
use super::material;
use super::debug;

pub struct Renderer<'a>
{
    camera: Option<&'a Camera>,
    scene: Option<&'a Scene<'a>>,

    antialias_samples: u16,
    scatter_limit: u16,

    debug: Option<debug::RenderDebug>,
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
            camera: None,
            scene: None,

            antialias_samples: 4,
            scatter_limit: 8,

            debug: None,
            debug_diffuse_material: material::Diffuse::from(Color::new(0.5, 0.5, 0.5, 1.0)),
            debug_normals_material: debug::VisualizeNormals::new(),
            stats: debug::RenderStats::new()
        }
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

    pub fn set_antialias_samples(mut self, count: u16) -> Self
    {
        self.antialias_samples = count;
        self
    }

    pub fn set_scatter_limit(mut self, count: u16) -> Self
    {
        self.scatter_limit = count;
        self
    }

    pub fn set_debug(mut self, debug: debug::RenderDebug) -> Self
    {
        self.debug = Some(debug);
        self
    }

    pub fn render(mut self, image: &mut Image) -> Self
    {
        let begin_time = std::time::Instant::now();

        let camera = self.camera.expect("Cannot render image without camera!");

        debug_assert!(self.antialias_samples > 0);
        let antialias_subpixel_step = 1.0 / self.antialias_samples as f32;

        for y in 0..image.get_height()
        {
            for x in 0..image.get_width()
            {
                self.stats.pixels += 1;

                let mut color = Color::new(0.0, 0.0, 0.0, 0.0);

                for subpixel_x in 0..self.antialias_samples
                {
                    for subpixel_y in 0..self.antialias_samples
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

                color /= self.antialias_samples.pow(2);

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

        self
    }

    fn sample(&mut self, ray: Ray, scatter_index: u16) -> Color
    {
        let scene = self.scene.expect("Cannot render image without scene!");

        if scatter_index > self.scatter_limit
        {
            return Color::black();
        }

        self.stats.samples += 1;
        
        if let Some(intersection) = scene.intersect(&ray, 0.001, std::f32::MAX)
        {
            self.stats.intersections += 1;
            
            let material = match self.debug
            {
                None => intersection.material,
                Some(debug::RenderDebug::Diffuse) => &self.debug_diffuse_material,
                Some(debug::RenderDebug::Normals) => &self.debug_normals_material,
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
            match self.debug
            {
                Some(debug::RenderDebug::Diffuse) => return Color::new(0.5, 0.5, 0.5, 1.0),
                Some(debug::RenderDebug::Normals) => return Color::new(0.5, 0.5, 1.0, 1.0),
                None =>
                {
                    let alpha = (ray.get_direction().y + 1.0) * 0.5;
                    return Color::new(1.0, 1.0, 1.0, 1.0).mul_rgb(1.0 - alpha).add_rgb(Color::new(0.5, 0.7, 1.0, 1.0).mul_rgb(alpha));
                }
            };
        }
    }

    pub fn print_stats(self) -> Self
    {
        self.stats.print();
        self
    }
}
