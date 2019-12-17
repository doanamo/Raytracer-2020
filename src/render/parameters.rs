use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub enum DebugMode
{
    Diffuse,
    Normals,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters
{
    pub image_width: usize,
    pub image_height: usize,
    pub antialias_samples: u16,
    pub scatter_limit: u16,
    pub debug_mode: Option<DebugMode>
}

impl Default for Parameters
{
    fn default() -> Self
    {
        Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 4,
            scatter_limit: 8,
            debug_mode: None
        }
    }
}
