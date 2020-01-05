use std::io::Write;
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::path::Path;

use super::surface::Surface;
use super::writer::Format;
use super::writer::Error;

#[derive(Default)]
pub struct FormatPNM
{
}

impl FormatPNM
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn save(surface: &Surface, path: &Path) -> std::io::Result<()>
    {
        let mut image_buffer = BufWriter::new(OpenOptions::new()
            .write(true).truncate(true).create(true).open(path)?);

        writeln!(image_buffer, "P3")?;
        writeln!(image_buffer, "{} {}", surface.get_width(), surface.get_height())?;
        writeln!(image_buffer, "255")?;

        for y in (0..surface.get_height()).rev()
        {
            for x in 0..surface.get_width()
            {
                let color = surface.get_pixel(x, y).as_quantized_u8_array();
                writeln!(image_buffer, "{} {} {}", color[0], color[1], color[2])?;
            }
        }

        Ok(())
    }
}

impl Format for FormatPNM
{
    fn save(&self, surface: &Surface, path: &Path) -> Result<(), Error>
    {
        Self::save(surface, path).or(Err(Error::SaveFailed))
    }

    fn get_name(&self) -> &'static str
    {
        "PNM"
    }
}
