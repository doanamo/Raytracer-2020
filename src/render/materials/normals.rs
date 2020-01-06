use serde::{ Serialize, Deserialize };
use super::math::Color;
use super::math::Ray;
use super::math::Intersection;
use super::Material;

#[derive(Default, Serialize, Deserialize)]
pub struct Normals
{
}

impl Normals
{
    pub fn new() -> Material
    {
        Material::Normals(Self::default())
    }

    pub fn scatter(&self, _ray: &Ray, intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Color)
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
