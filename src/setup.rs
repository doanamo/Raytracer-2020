use std::fs::OpenOptions;
use std::io::BufWriter;
use serde::{ Serialize, Deserialize };
use super::render;

#[derive(Debug)]
pub enum SetupError
{
    OpeningFileFailed,
    SerializationFailed,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Setup
{
    pub parameters: render::Parameters,
    pub camera: render::Camera,
    pub scene: render::Scene
}

impl Setup
{
    pub fn new() -> Self
    {
        Setup::default()
    }

    pub fn save(&self, path: &str) -> Result<(), SetupError>
    {
        let setup_file = OpenOptions::new().write(true).truncate(true).create(true).open(path).or(Err(SetupError::OpeningFileFailed))?;
        let file_writer = BufWriter::new(setup_file);

        serde_json::to_writer_pretty(file_writer, &self).or(Err(SetupError::SerializationFailed))?;

        Ok(())
    }
}
