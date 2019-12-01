use serde::{ Serialize, Deserialize };
use crate::math::Ray;
use crate::math::Color;
use super::primitive::Intersection;
use super::material::Material;

#[derive(Serialize, Deserialize)]
pub enum DebugMode
{
    Diffuse,
    Normals,
}

pub struct RenderStats
{
    pub pixels: usize,
    pub subpixels: usize,
    pub samples: usize,
    pub intersections: usize,
    pub scatters: usize,
    pub max_scatters: u16
}

impl RenderStats
{
    pub fn new() -> Self
    {
        RenderStats
        {
            pixels: 0,
            subpixels: 0,
            samples: 0,
            intersections: 0,
            scatters: 0,
            max_scatters: 0
        }
    }

    pub fn print(&self)
    {
        println!("Printing render stats:");
        println!("  Pixels:        {}", self.pixels);
        println!("  Subpixels:     {} ({} per pixel)", self.subpixels,self.subpixels / self.pixels);
        println!("  Samples:       {} ({:.2} per pixel)", self.samples, self.samples as f32 / self.pixels as f32);
        println!("  Intersections: {} ({:.2} per pixel)", self.intersections, self.intersections as f32 / self.pixels as f32);
        println!("  Scatters:      {} ({} max)", self.scatters, self.max_scatters);
    }
}

#[derive(Serialize, Deserialize)]
pub struct VisualizeNormals
{
}

impl VisualizeNormals
{
    pub fn new() -> Self
    {
        VisualizeNormals
        {
        }
    }
}

#[typetag::serde]
impl Material for VisualizeNormals
{
    fn scatter(&self, _ray: &Ray, intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Color)
    {
        if scatter_index == 0
        {
            let normal_color = Color::new(intersection.normal.x + 1.0, intersection.normal.y + 1.0, intersection.normal.z + 1.0, 1.0).mul_rgb(0.5);

            (None, normal_color)
        }
        else
        {
            panic!("Did not expect debug material for normals to scatter!");
        }
    }
}
