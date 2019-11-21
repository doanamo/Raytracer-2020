use crate::math::*;
use crate::image::Image;
use super::camera::Camera;
use super::scene::Scene;

pub enum RenderDebug
{
    Normals,
}

pub struct RenderStats
{
    pixels: usize,
    subpixels: usize,
    samples: usize,
    bounces: usize,
    max_bounces: u16
}

impl RenderStats
{
    pub fn new() -> Self
    {
        RenderStats
        {
            pixels: 0,
            subpixels: 0,
            samples: 0,
            bounces: 0,
            max_bounces: 0
        }
    }

    pub fn print(&self)
    {
        println!("Printing render stats:");
        println!("  Pixels:    {}", self.pixels);
        println!("  Subpixels: {}", self.subpixels);
        println!("  Samples:   {}", self.samples);
        println!("  Bounces:   {} ({} max)", self.bounces, self.max_bounces);
    }
}

pub struct Renderer<'a>
{
    camera: Option<&'a Camera>,
    scene: Option<&'a Scene<'a>>,

    antialias_samples: u16,
    bounce_limit: u16,

    debug: Option<RenderDebug>,
    stats: RenderStats
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
            bounce_limit: 8,

            debug: None,
            stats: RenderStats::new()
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

    pub fn set_bounce_limit(mut self, count: u16) -> Self
    {
        self.bounce_limit = count;
        self
    }

    pub fn set_debug(mut self, debug: RenderDebug) -> Self
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

    fn sample(&mut self, ray: Ray, bounce_index: u16) -> Color
    {
        if bounce_index > self.bounce_limit
        {
            return Color::new(0.0, 0.0, 0.0, 1.0);
        }

        self.stats.samples += 1;
        self.stats.max_bounces = std::cmp::max(self.stats.max_bounces, bounce_index);

        let scene = self.scene.expect("Cannot render image without scene!");
        let intersection = scene.intersect(&ray, 0.001, std::f32::MAX);

        if let Some(intersection) = intersection
        {
            match self.debug
            {
                None =>
                {
                    if let Some((scattered_ray, attenuation)) = intersection.material.scatter(&intersection)
                    {
                        self.stats.bounces += 1;
                        return self.sample(scattered_ray, bounce_index + 1) * attenuation;
                    }
                    else
                    {
                        return Color::new(0.0, 0.0, 0.0, 1.0);
                    }
                },
                Some(RenderDebug::Normals) =>
                {
                    return Color::new(intersection.normal.x + 1.0, intersection.normal.y + 1.0, intersection.normal.z + 1.0, 1.0).mul_rgb(0.5);
                }
            }
        }
        else
        {
            let alpha = (ray.get_direction().y + 1.0) * 0.5;
            return Color::new(1.0, 1.0, 1.0, 1.0).mul_rgb(1.0 - alpha).add_rgb(Color::new(0.5, 0.7, 1.0, 1.0).mul_rgb(alpha));
        }
    }

    pub fn print_stats(self) -> Self
    {
        self.stats.print();
        self
    }
}
