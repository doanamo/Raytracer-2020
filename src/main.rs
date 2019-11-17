mod math;
mod render;
mod image;

use math::Vec3;
use image::Image;
use render::Sphere;

fn main() 
{
    let mut image = Image::new(1024, 576);

    render::Scene::new()
        .add_primitive(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)))
        .add_primitive(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)))
        .render(&mut image);

    image::Writer::new(Box::new(image::FormatPNM::new()))
        .input(&image).output("output/render.pnm").save()
        .expect("Failed to save rendered image!");

    image::Writer::new(Box::new(image::FormatPNG::new()))
        .input(&image).output("output/render.png").save()
        .expect("Failed to save rendered image!");
}
