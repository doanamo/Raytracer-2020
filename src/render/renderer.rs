use crate::math::Color;
use crate::math::Ray;
use crate::image::Image;
use super::camera::Camera;
use super::scene::Scene;
use super::primitive::Intersection;

pub struct Renderer<'a>
{
    camera: Option<&'a Camera>,
    scene: Option<&'a Scene>
}

impl<'a> Renderer<'a>
{
    pub fn new() -> Self
    {
        Renderer
        {
            camera: None,
            scene: None
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

    pub fn render(&self, image: &mut Image)
    {
        let begin_time = std::time::Instant::now();

        let camera = self.camera.expect("Cannot render image without camera!");
        let scene = self.scene.expect("Cannot render image without scene!");

        let antialias_samples = 4;
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
                        
                        let ray = camera.calculate_ray(u, v);
                        let intersection = scene.intersect(&ray, 0.0, std::f32::MAX);

                        color += self.sample(&ray, &intersection);
                    }
                }

                color /= antialias_samples * antialias_samples;
                assert!(color.is_valid());

                image.set_pixel(x, y, color);
            }
        }

        println!("Rendered image in {} seconds.", begin_time.elapsed().as_secs_f32());
    }

    fn sample(&self, ray: &Ray, intersection: &Option<Intersection>) -> Color
    {
        if let Some(intersection) = intersection
        {
            return Color::new(intersection.normal.x + 1.0, intersection.normal.y + 1.0, intersection.normal.z + 1.0, 1.0).mul_rgb(0.5);
        }
        else
        {
            let alpha = (ray.direction.normalized().y + 1.0) * 0.5;
            return Color::new(1.0, 1.0, 1.0, 1.0).mul_rgb(1.0 - alpha).add_rgb(Color::new(0.5, 0.7, 1.0, 1.0).mul_rgb(alpha));
        }
    }
}
