#![allow(dead_code)]

mod setup;
mod math;
mod render;
mod image;

use setup::Setup;

fn main() 
{
    let arguments: Vec<String> = std::env::args().collect();

    let setup = Setup::from_file("scenes/spheres.json").expect("Loading scene file failed!");

    {
        let image = render::Renderer::new()
            .set_parameters(&setup.parameters)
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
                .set_scene(&setup.scene)
                .render();
        
            image::Writer::new(image::FormatPNG::new())
                .input(&image).output("output/debug_normals.png")
                .save().expect("Failed to save rendered image!");
        }
    }
}
