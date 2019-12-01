#![allow(dead_code)]

mod setup;
mod math;
mod render;
mod image;

use setup::Setup;
use math::Vec3;
use math::Color;
use render::primitive;
use render::material;

fn main() 
{
    let arguments: Vec<String> = std::env::args().collect();

    let parameters = render::Parameters
    {
        image_width: 1024,
        image_height: 576,
        antialias_samples: 8,
        scatter_limit: 32,
        ..render::Parameters::default()
    };

    let camera = render::Camera::new()
        .set_origin(Vec3::new(0.0, -0.6, 0.0))
        .set_look_at(Some(Vec3::new(0.0, 1.0, -0.2)))
        .set_aspect_ratio_from_size(parameters.image_width, parameters.image_height)
        .set_field_of_view(55.0)
        .build();
    
    let scene = render::Scene::new()
        .add_primitive(primitive::Sphere::new(Vec3::new(0.3, 0.5, -0.3), 0.2, material::Refractive::from(Color::new(1.0, 1.0, 1.0, 1.0), 1.008)))
        .add_primitive(primitive::Sphere::new(Vec3::new(-0.3, 0.5, -0.3), -0.2, material::Refractive::from(Color::new(1.0, 1.0, 1.0, 1.0), 1.3)))
        .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 1.4, 0.0), 0.5, material::Diffuse::from(Color::new(0.8, 0.3, 0.3, 1.0))))
        .add_primitive(primitive::Sphere::new(Vec3::new(0.8, 1.0, -0.1), 0.4, material::Metalic::from(Color::new(0.8, 0.8, 0.8, 1.0), 0.0)))
        .add_primitive(primitive::Sphere::new(Vec3::new(-0.8, 1.0, -0.1), 0.4, material::Metalic::from(Color::new(0.8, 0.8, 0.8, 1.0), 0.8)))
        .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, material::Diffuse::from(Color::new(0.8, 0.8, 0.0, 1.0))));

    let setup = Setup
    {
        parameters,
        camera,
        scene
    };

    setup.save("output/render.json").expect("Saving setup file failed!");

    {
        let image = render::Renderer::new()
            .set_parameters(&setup.parameters)
            .set_camera(&setup.camera)
            .set_scene(&setup.scene)
            .render();
    
        image::Writer::new(image::FormatPNG::new())
            .input(&image).output("output/render.png")
            .save().expect("Failed to save rendered image!");
    }

    if arguments.iter().any(|arg| arg == "--render_debug")
    {
        {
            let custom_parameters = render::Parameters
            {
                image_width: 1025,
                image_height: 576,
                antialias_samples: 2,
                scatter_limit: 8,
                debug_mode: Some(render::DebugMode::Diffuse)
            };

            let image = render::Renderer::new()
                .set_parameters(&custom_parameters)
                .set_camera(&setup.camera)
                .set_scene(&setup.scene)
                .render();
        
            image::Writer::new(image::FormatPNG::new())
                .input(&image).output("output/debug_diffuse.png")
                .save().expect("Failed to save rendered image!");
        }

        {
            let custom_parameters = render::Parameters
            {
                image_width: 1025,
                image_height: 576,
                antialias_samples: 2,
                scatter_limit: 1,
                debug_mode: Some(render::DebugMode::Diffuse)
            };

            let image = render::Renderer::new()
                .set_parameters(&custom_parameters)
                .set_camera(&setup.camera)
                .set_scene(&setup.scene)
                .render();
        
            image::Writer::new(image::FormatPNG::new())
                .input(&image).output("output/debug_normals.png")
                .save().expect("Failed to save rendered image!");
        }
    }
}
