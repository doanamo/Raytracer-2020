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
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Material
    {
        Material::Normals(Self::default())
    }

    pub fn scatter(intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Color)
    {
        debug_assert!(scatter_index == 0, "Did not expect debug material for normals to scatter!");

        let normal_color = Color::new(intersection.normal.x + 1.0, intersection.normal.y + 1.0, intersection.normal.z + 1.0, 1.0).mul_rgb(0.5);

        (None, normal_color)
    }
}
