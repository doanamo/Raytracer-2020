use serde::{ Serialize, Deserialize };
use super::math::Vec4;
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

    pub fn scatter(intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Vec4)
    {
        debug_assert!(scatter_index == 0, "Did not expect debug material for normals to scatter!");

        let normal_color = Vec4::new
        (
            (intersection.normal.get_x() + 1.0) * 0.5,
            (intersection.normal.get_y() + 1.0) * 0.5,
            (intersection.normal.get_z() + 1.0) * 0.5,
            1.0
        );

        (None, normal_color)
    }
}
