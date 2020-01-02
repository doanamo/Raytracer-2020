use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;

use super::Surface;
use super::Format;
use super::Error;

#[derive(Default)]
pub struct FormatPNG
{
}

impl FormatPNG
{
    pub fn new() -> FormatPNG
    {
        FormatPNG::default()
    }

    pub fn save(&self, surface: &Surface, path: &Path) -> Result<(), Error>
    {
        let surface_byte_count = surface.get_pixel_count() * std::mem::size_of::<u8>() * 4;
        let mut surface_bytes: Vec<u8> = Vec::with_capacity(surface_byte_count);

        for y in (0..surface.get_height()).rev()
        {
            for x in 0..surface.get_width()
            {
                let color = surface.get_pixel(x, y);
                surface_bytes.extend(color.as_quantized_u8_array().iter());
            }
        }

        debug_assert_eq!(surface_bytes.len(), surface_byte_count);

        let image_file = OpenOptions::new().write(true).truncate(true).create(true).open(path).or(Err(Error::SaveFailed))?;
        let image_buffer = BufWriter::new(image_file);

        let mut image_encoder = png::Encoder::new(image_buffer, surface.get_width() as u32, surface.get_height() as u32);
        image_encoder.set_color(png::ColorType::RGBA);
        image_encoder.set_depth(png::BitDepth::Eight);
        
        let mut image_writer = image_encoder.write_header().or(Err(Error::SaveFailed))?;
        image_writer.write_image_data(surface_bytes.as_slice()).or(Err(Error::SaveFailed))?;

        Ok(())
    }
}

impl Format for FormatPNG
{
    fn save(&self, surface: &Surface, path: &Path) -> Result<(), Error>
    {
        FormatPNG::save(self, surface, path).or(Err(Error::SaveFailed))
    }

    fn get_name(&self) -> &'static str
    {
        "PNG"
    }
}
