use crate::math::Color;
use crate::math::Vec3;
use crate::math::Ray;
use crate::image::Image;
use super::camera::Camera;
use super::scene::Scene;

pub enum RenderDebug
{
    Normals,
}

pub struct Renderer<'a>
{
    camera: Option<&'a Camera>,
    scene: Option<&'a Scene>,
    debug: Option<RenderDebug>
}

impl<'a> Renderer<'a>
{
    pub fn new() -> Self
    {
        Renderer
        {
            camera: None,
            scene: None,
            debug: None
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

    pub fn set_debug(mut self, debug: RenderDebug) -> Self
    {
        self.debug = Some(debug);
        self
    }

    pub fn render(&self, image: &mut Image)
    {
        let begin_time = std::time::Instant::now();

        let camera = self.camera.expect("Cannot render image without camera!");

        let antialias_samples = 8;
        let antialias_subpixel_step = 1.0 / antialias_samples as f32;

        for y in 0..image.get_height()
        {
            for x in 0..image.get_width()
            {
                let mut color = Color::new(0.0, 0.0, 0.0, 0.0);

                for subpixel_x in 0..antialias_samples
                {
                    for subpixel_y in 0..antialias_samples
                    {
                        let offset_u = subpixel_x as f32 * antialias_subpixel_step;
                        let offset_v = subpixel_y as f32 * antialias_subpixel_step;

                        let u = (x as f32 + offset_u) / image.get_width() as f32;
                        let v = (y as f32 + offset_v) / image.get_height() as f32;
                        
                        let sample = self.sample(camera.calculate_ray(u, v));
                        debug_assert!(sample.is_valid());

                        color += sample;
                    }
                }

                color /= antialias_samples * antialias_samples;

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
    }

    fn sample(&self, ray: Ray) -> Color
    {
        let scene = self.scene.expect("Cannot render image without scene!");
        let intersection = scene.intersect(&ray, 0.001, std::f32::MAX);

        if let Some(intersection) = intersection
        {
            if let Some(debug) = &self.debug
            {
                match debug
                {
                    RenderDebug::Normals =>
                    {
                        return Color::new(intersection.normal.x + 1.0, intersection.normal.y + 1.0, intersection.normal.z + 1.0, 1.0).mul_rgb(0.5);
                    }
                }
            }
            else
            {
                let bounce_target = intersection.point + intersection.normal + Vec3::random_in_unit_sphere();
                let bounce_ray = Ray::new(intersection.point, (bounce_target - intersection.point).normalized());
                return self.sample(bounce_ray).mul_rgb(0.5);
            }
        }
        else
        {
            let alpha = (ray.get_direction().y + 1.0) * 0.5;
            return Color::new(1.0, 1.0, 1.0, 1.0).mul_rgb(1.0 - alpha).add_rgb(Color::new(0.5, 0.7, 1.0, 1.0).mul_rgb(alpha));
        }
    }
}
