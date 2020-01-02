use std::path::Path;
use std::fs::OpenOptions;
use std::io::{ BufWriter, BufReader };
use serde::{ Serialize, Deserialize };
use super::parameters::Parameters;
use super::scene::Scene;

#[derive(Debug)]
pub enum Error
{
    OpeningFile,
    CreatingFile,
    Serializing,
    Deserializing,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Setup
{
    pub parameters: Parameters,
    pub scene: Scene
}

impl Setup
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error>
    {
        let scene_file = OpenOptions::new().read(true).open(path).or(Err(Error::OpeningFile))?;
        let file_reader = BufReader::new(scene_file);

        match serde_json::from_reader(file_reader)
        {
            Ok(setup) => Ok(setup),
            Err(error) =>
            {
                println!("Deserialization error: {}", error);
                Err(Error::Deserializing)
            }
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error>
    {
        match path.as_ref().parent()
        {
            Some(directory) =>
            {
                let _ = std::fs::create_dir_all(directory);
            },
            None => return Err(Error::CreatingFile)
        };

        let scene_file = OpenOptions::new().write(true).truncate(true).create(true).open(path).or(Err(Error::CreatingFile))?;
        let file_writer = BufWriter::new(scene_file);

        match serde_json::to_writer_pretty(file_writer, &self)
        {
            Ok(()) => Ok(()),
            Err(error) =>
            {
                println!("Serialization error: {}", error);
                Err(Error::Serializing)
            }
        }
    }
}
