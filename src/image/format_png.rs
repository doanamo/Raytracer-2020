use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;

use crate::image::Image;
use crate::image::writer::Format;
use crate::image::writer::Error;

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

    pub fn save(&self, image: &Image, path: &Path) -> Result<(), Error>
    {
        let mut image_bytes: Vec<u8> = Vec::with_capacity(image.get_pixel_count());

        for y in (0..image.get_height()).rev()
        {
            for x in 0..image.get_width()
            {
                let color = image.get_pixel(x, y);
                image_bytes.extend(color.as_quantized_u8_array().iter());
            }
        }

        debug_assert_eq!(image_bytes.len(), image.get_pixel_count());

        let image_file = OpenOptions::new().write(true).truncate(true).create(true).open(path).or(Err(Error::SaveFailed))?;
        let image_buffer = BufWriter::new(image_file);

        let mut image_encoder = png::Encoder::new(image_buffer, image.get_width() as u32, image.get_height() as u32);
        image_encoder.set_color(png::ColorType::RGBA);
        image_encoder.set_depth(png::BitDepth::Eight);
        
        let mut image_writer = image_encoder.write_header().or(Err(Error::SaveFailed))?;
        image_writer.write_image_data(image_bytes.as_slice()).or(Err(Error::SaveFailed))?;

        Ok(())
    }
}

impl Format for FormatPNG
{
    fn save(&self, image: &Image, path: &Path) -> Result<(), Error>
    {
        FormatPNG::save(self, image, path).or(Err(Error::SaveFailed))
    }

    fn get_name(&self) -> &'static str
    {
        "PNG"
    }
}
