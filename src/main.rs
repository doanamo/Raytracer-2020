mod math;
mod image;

use math::Color;
use math::Vec3;
use math::Ray;

use image::Image;

fn render_image(mut image: Image) -> Image
{
    let begin_time = std::time::Instant::now();

    // Correct aspect ratio
    let mut view_width = 4.0;
    let mut view_height = 2.0;

    if image.get_width() > image.get_height()
    {
        let source_width_aspect_ratio = view_width / view_height;
        let target_width_aspect_ratio = image.get_width() as f32 / image.get_height() as f32;
        view_width *= target_width_aspect_ratio / source_width_aspect_ratio;
    }
    else
    {
        let source_height_aspect_ratio = view_height / view_width;
        let target_height_aspect_ratio = image.get_height() as f32 / image.get_width() as f32;
        view_height *= target_height_aspect_ratio / source_height_aspect_ratio;
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
            let hit = ray.intersect_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5);

            if hit > 0.0
            {
                let normal = (ray.point_at(hit) - Vec3::new(0.0, 0.0, -1.0)).normalized();
                color = Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0, 1.0).mul_rgb(0.5);
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

    image
}

fn main() 
{
    let image = render_image(Image::new(1920, 1080));

    image::Writer::new(Box::new(image::FormatPNM::new()))
        .input(&image).output("output/image.pnm").save()
        .expect("Failed to save rendered image!");
}
