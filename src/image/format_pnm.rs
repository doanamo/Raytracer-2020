use std::io::Write;
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::path::Path;

use super::Surface;
use super::Format;
use super::Error;

#[derive(Default)]
pub struct FormatPNM
{
}

impl FormatPNM
{
    pub fn new() -> FormatPNM
    {
        FormatPNM::default()
    }

    pub fn save(&self, surface: &Surface, path: &Path) -> std::io::Result<()>
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
                let color = surface.get_pixel(x, y);

                let ir = (color.r * 255.99).floor() as i32;
                let ig = (color.g * 255.99).floor() as i32;
                let ib = (color.b * 255.99).floor() as i32;

                writeln!(image_buffer, "{} {} {}", ir, ig, ib)?;
            }
        }

        Ok(())
    }
}

impl Format for FormatPNM
{
    fn save(&self, surface: &Surface, path: &Path) -> Result<(), Error>
    {
        FormatPNM::save(self, surface, path).or(Err(Error::SaveFailed))
    }

    fn get_name(&self) -> &'static str
    {
        "PNM"
    }
}
