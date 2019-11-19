use crate::math::Color;
use crate::math::Vec3;
use crate::math::Ray;
use crate::image::Image;
use super::scene::Scene;

pub struct Renderer<'a>
{
    scene: Option<&'a Scene>
}

impl<'a> Renderer<'a>
{
    pub fn new() -> Self
    {
        Renderer
        {
            scene: None
        }
    }

    pub fn set_scene(mut self, scene: &'a Scene) -> Self
    {
        self.scene = Some(scene);
        self
    }

    pub fn render(&self, image: &mut Image)
    {
        let scene = self.scene.expect("Cannot render image without scene!");

        let begin_time = std::time::Instant::now();

        // Correct aspect ratio
        let mut view_width = 2.0;
        let mut view_height = 2.0;

        let source_aspect_ratio = image.get_width() as f32 / image.get_height() as f32;
        let target_aspect_ratio = view_width / view_height;

        if source_aspect_ratio > target_aspect_ratio
        {
            view_width *= source_aspect_ratio / target_aspect_ratio;
        }
        else
        {
            view_height *= target_aspect_ratio / source_aspect_ratio;
        }

        // Calculate view space
        let bottom_left = Vec3::new(view_width * -0.5, view_height * -0.5, -1.0);
        let horizontal_width = Vec3::new(view_width, 0.0, 0.0);
        let vertical_height = Vec3::new(0.0, view_height, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        // Render image pixel by pixel
        for y in 0..image.get_height()
        {
            for x in 0..image.get_width()
            {
                let color;

                let u = x as f32 / image.get_width() as f32;
                let v = y as f32 / image.get_height() as f32;

                let ray = Ray::new(origin, bottom_left + horizontal_width * u + vertical_height * v);

                if let Some(intersection) = scene.intersect(&ray, 0.0, std::f32::MAX)
                {
                    color = Color::new(intersection.normal.x + 1.0, intersection.normal.y + 1.0, intersection.normal.z + 1.0, 1.0).mul_rgb(0.5);
                }
                else
                {
                    let alpha = (ray.direction.normalized().y + 1.0) * 0.5;
                    color = Color::new(1.0, 1.0, 1.0, 1.0).mul_rgb(1.0 - alpha).add_rgba(Color::new(0.5, 0.7, 1.0, 1.0).mul_rgb(alpha));
                }

                image.set_pixel(x, y, color);
            }
        }

        println!("Rendered image in {} seconds.", begin_time.elapsed().as_secs_f32());
    }
}
