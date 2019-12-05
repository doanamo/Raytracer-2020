use std::fs::OpenOptions;
use std::io::{ BufWriter, BufReader };
use serde::{ Serialize, Deserialize };
use super::render;

#[derive(Debug)]
pub enum Error
{
    OpeningFileFailed,
    CreatingFileFailed,

    SerializationFailed,
    DeserializationFailed,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Setup
{
    pub parameters: render::Parameters,
    pub scene: render::Scene
}

impl Setup
{
    pub fn new() -> Self
    {
        Setup::default()
    }

    pub fn from_file(path: &str) -> Result<Self, Error>
    {
        let scene_file = OpenOptions::new().read(true).open(path).or(Err(Error::OpeningFileFailed))?;
        let file_reader = BufReader::new(scene_file);

        match serde_json::from_reader(file_reader)
        {
            Err(error) =>
            {
                println!("Deserialization error: {}", error);
                Err(Error::DeserializationFailed)
            },
            Ok(setup) => Ok(setup)
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Error>
    {
        let scene_file = OpenOptions::new().write(true).truncate(true).create(true).open(path).or(Err(Error::CreatingFileFailed))?;
        let file_writer = BufWriter::new(scene_file);

        match serde_json::to_writer_pretty(file_writer, &self)
        {
            Err(error) =>
            {
                println!("Serialization error: {}", error);
                Err(Error::SerializationFailed)
            },
            Ok(setup) => Ok(setup)
        }
    }
}
