use std::io::Write;
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::image::Image;

pub struct WriterPNM
{
    image: Image,
    output: PathBuf
}

impl WriterPNM
{
    pub fn new(image: Image) -> WriterPNM
    {
        WriterPNM
        {
            image: image,
            output: PathBuf::new()
        }
    }

    pub fn output(mut self, path: &str) -> Self
    {
        self.output = PathBuf::from(path);
        self
    }
    
    pub fn save(&self) -> std::io::Result<()>
    {
        let begin_time = std::time::Instant::now();

        let _ = std::fs::create_dir_all(&self.output);
        let mut image_buffer = BufWriter::new(OpenOptions::new()
            .write(true).truncate(true).create(true).open(&self.output)?);

        writeln!(image_buffer, "P3")?;
        writeln!(image_buffer, "{} {}", self.image.get_width(), self.image.get_height())?;
        writeln!(image_buffer, "255")?;

        for y in (0..self.image.get_height()).rev()
        {
            for x in 0..self.image.get_width()
            {
                let color = self.image.get_pixel(x, y);

                let ir = (color.r * 255.99).floor() as i32;
                let ig = (color.g * 255.99).floor() as i32;
                let ib = (color.b * 255.99).floor() as i32;

                writeln!(image_buffer, "{} {} {}", ir, ig, ib)?;
            }
        }

        println!("Saved image in {} seconds.", begin_time.elapsed().as_secs_f32());

        Ok(())
    }
}
