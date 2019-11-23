#![allow(dead_code)]

mod math;
mod render;
mod image;

use math::Vec3;
use math::Color;
use image::Image;
use render::primitive;
use render::material;

fn main() 
{
    let image_width = 1024;
    let image_heigth = 576;

    let camera = render::Camera::new()
        .set_source_size(2.0, 2.0)
        .set_target_size(image_width, image_heigth)
        .build();
    
    let scene = render::Scene::new()
        .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material::Diffuse::from(Color::new(0.8, 0.3, 0.3, 1.0))))
        .add_primitive(primitive::Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material::Metalic::from(Color::new(0.8, 0.8, 0.8, 1.0), 0.8)))
        .add_primitive(primitive::Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material::Metalic::from(Color::new(0.8, 0.8, 0.8, 1.0), 0.0)))
        .add_primitive(primitive::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material::Diffuse::from(Color::new(0.8, 0.8, 0.0, 1.0))));

    {
        let mut render_image = Image::new(image_width, image_heigth);
        
        render::Renderer::new()
        .set_camera(&camera)
            .set_scene(&scene)
            .set_antialias_samples(8)
            .set_scatter_limit(128)
            .render(&mut render_image)
            .print_stats();
    
        image::Writer::new(image::FormatPNG::new())
            .input(&render_image).output("output/render.png").save()
            .expect("Failed to save rendered image!");
    }

    if true
    {
        {
            let mut render_debug_diffuse = Image::new(image_width, image_heigth);

            render::Renderer::new()
                .set_camera(&camera)
                .set_scene(&scene)
                .set_antialias_samples(2)
                .set_scatter_limit(8)
                .set_debug(render::RenderDebug::Diffuse)
                .render(&mut render_debug_diffuse);
        
            image::Writer::new(image::FormatPNG::new())
                .input(&render_debug_diffuse).output("output/debug_diffuse.png").save()
                .expect("Failed to save rendered image!");
        }

        {
            let mut render_debug_normals = Image::new(image_width, image_heigth);
    
            render::Renderer::new()
                .set_camera(&camera)
                .set_scene(&scene)
                .set_antialias_samples(2)
                .set_scatter_limit(1)
                .set_debug(render::RenderDebug::Normals)
                .render(&mut render_debug_normals);
        
            image::Writer::new(image::FormatPNG::new())
                .input(&render_debug_normals).output("output/debug_normals.png").save()
                .expect("Failed to save rendered image!");
        }
    }
}
