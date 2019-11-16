use crate::image::Image;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SaveError
{
    MissingInput,
    MissingOutput,
    InvalidPath,
    SaveFailed
}

pub trait Format
{
    fn save(&self, image: &Image, path: &PathBuf) -> Result<(), SaveError>;
    fn get_name(&self) -> &'static str;
}

pub struct Writer<'a>
{
    format: Box<dyn Format>,
    image: Option<&'a Image>,
    path: Option<PathBuf>
}

impl<'a> Writer<'a>
{
    pub fn new(format: Box<dyn Format>) -> Writer<'a>
    {
        Writer
        {
            format: format,
            image: None,
            path: None
        }
    }

    pub fn input(mut self, image: &'a Image) -> Self
    {
        self.image = Some(image);
        self
    }

    pub fn output(mut self, path: &str) -> Self
    {
        self.path = Some(PathBuf::from(path));
        self
    }

    pub fn save(self) -> Result<(), SaveError>
    {
        let image = self.image.ok_or(SaveError::MissingInput)?;
        let path = self.path.as_ref().ok_or(SaveError::MissingOutput)?;

        {
            let begin_time = std::time::Instant::now();

            match path.parent()
            {
                None => return Err(SaveError::InvalidPath),
                Some(directory) =>
                {
                    let _ = std::fs::create_dir_all(directory);
                }
            };

            self.format.save(image, path)?;

            println!("Saved {} file in {} seconds.", self.format.get_name(), begin_time.elapsed().as_secs_f32());
        }

        Ok(())
    }
}
