#![allow(dead_code)]

mod setup;

use std::path::{ Path, PathBuf };
use clap::{ App, Arg };
use setup::Setup;
use raytracer::render;
use raytracer::image;

fn main() 
{
    // Declare application entry.
    let options = App::new("Raytracer")
        .version("0.1")
        .author("Piotr Doan <doanpiotr@gmail.com>")
        .about("Raytracer written in Rust.")
        .arg(Arg::with_name("input")
            .index(1).required(true)
            .short("i").long("input")
            .help("Path to input setup file in JSON format"))
        .arg(Arg::with_name("output")
            .index(2)
            .short("o").long("output")
            .help("Path to output image file in PNG format"))
        .get_matches();

    // Parse input setup file path from arguments.
    let input_setup_path = Path::new(options.value_of("input").unwrap());
    let input_setup_filename = input_setup_path.file_name().expect("Expected input path that points to a file!");
    println!("Input setup file: {}", input_setup_path.display());

    // Create output setup file path.
    let output_setup_path = PathBuf::from("output/file").with_file_name(&input_setup_filename).with_extension("json");
    println!("Output setup file: {}", output_setup_path.display());

    // Parse output image file path from arguments.
    let output_image_path = match options.value_of("output")
    {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("output/file").with_file_name(&input_setup_filename).with_extension("png")
    };

    println!("Output image file: {}", output_image_path.display());

    // Load setup from input file.
    let setup = Setup::from_file(&input_setup_path).expect("Loading setup file failed!");

    // Save setup to output file.
    // This can be helpful if we will want to convert old setup file to new format or compare both.
    setup.save(&output_setup_path).expect("Saving setup file failed!");
    
    // Render image.
    let image = render::Renderer::new()
        .set_parameters(&setup.parameters)
        .set_scene(&setup.scene)
        .render();

    // Save rendered image.
    image::Writer::new(image::FormatPNG::new())
        .input(&image).output(&output_image_path)
        .save().expect("Failed to save rendered image!");
}
