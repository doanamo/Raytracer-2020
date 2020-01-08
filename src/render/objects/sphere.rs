use serde::{ Serialize, Deserialize };
use super::math::Vec3;
use super::math::geometry;
use super::materials::Material;
use super::Object;

#[derive(Serialize, Deserialize)]
pub struct Sphere
{
    #[serde(flatten)]
    pub shape: geometry::Sphere,
    pub material: Material
}

impl Sphere
{
    #[allow(clippy::new_ret_no_self)]
    pub fn new(center: Vec3, radius: f32, material: Material) -> Object
    {
        Object::Sphere(Self
        {
            shape: geometry::Sphere
            {
                center,
                radius
            },
            material
        })
    }
}
