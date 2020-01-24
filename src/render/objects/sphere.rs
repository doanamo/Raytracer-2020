use serde::{ Serialize, Deserialize };
use super::math::Vec3;
use super::math::geometry;
use super::materials::Material;
use super::ObjectKind;

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
    pub fn new(center: Vec3, radius: f32, material: Material) -> ObjectKind
    {
        ObjectKind::Sphere(Self
        {
            shape: geometry::Sphere
            {
                center,
                radius
            },
            material
        })
    }

    pub fn at_time(&self, time: f32, velocity: Vec3) -> geometry::Sphere
    {
        geometry::Sphere
        {
            center: self.shape.center + velocity * time,
            radius: self.shape.radius
        }
    }
}
