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

    pub fn save(&self, image: &Image, path: &PathBuf) -> Result<(), lodepng::ffi::Error>
    {
        let mut image_data: Vec<u8> = Vec::new();

        for y in (0..image.get_height()).rev()
        {
            for x in 0..image.get_width()
            {
                let color = image.get_pixel(x, y);
                image_data.extend(color.as_u8_array().iter());
            }
        }

        lodepng::encode32_file(path, image_data.as_slice(), image.get_width(), image.get_height())?;

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
