use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::PathBuf;

use crate::image::Image;
use crate::image::writer::Format;
use crate::image::writer::SaveError;

pub struct FormatPNG
{
}

impl FormatPNG
{
    pub fn new() -> FormatPNG
    {
        FormatPNG
        {
        }
    }

    pub fn save(&self, image: &Image, path: &PathBuf) -> Result<(), SaveError>
    {
        let mut image_data: Vec<u8> = Vec::with_capacity(image.get_width() * image.get_height());

        for y in (0..image.get_height()).rev()
        {
            for x in 0..image.get_width()
            {
                let color = image.get_pixel(x, y);
                image_data.extend(color.as_u8_array().iter());
            }
        }

        let image_file = OpenOptions::new().write(true).truncate(true).create(true).open(path).or(Err(SaveError::SaveFailed))?;
        let image_buffer = BufWriter::new(image_file);

        let mut image_encoder = png::Encoder::new(image_buffer, image.get_width() as u32, image.get_height() as u32);
        image_encoder.set_color(png::ColorType::RGBA);
        image_encoder.set_depth(png::BitDepth::Eight);

        let mut image_writer = image_encoder.write_header().or(Err(SaveError::SaveFailed))?;
        image_writer.write_image_data(image_data.as_slice()).or(Err(SaveError::SaveFailed))?;

        Ok(())
    }
}

impl Format for FormatPNG
{
    fn save(&self, image: &Image, path: &PathBuf) -> Result<(), SaveError>
    {
        FormatPNG::save(self, image, path).or(Err(SaveError::SaveFailed))
    }

    fn get_name(&self) -> &'static str
    {
        "PNG"
    }
}
