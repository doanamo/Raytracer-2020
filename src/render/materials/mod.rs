use serde::{ Serialize, Deserialize };

use super::math;
use super::math::Vec4;
use super::math::Ray;
use super::math::Intersection;

pub mod diffuse;
pub use diffuse::Diffuse;
pub mod normals;
pub use normals::Normals;
pub mod metallic;
pub use metallic::Metallic;
pub mod refractive;
pub use refractive::Refractive;

#[derive(Serialize, Deserialize)]
pub enum Material
{
    Diffuse(Diffuse),
    Normals(Normals),
    Metallic(Metallic),
    Refractive(Refractive)
}

impl Material
{
    pub fn scatter(&self, ray: &Ray, intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Vec4)
    {
        match &self
        {
            Self::Diffuse(diffuse) => diffuse.scatter(intersection),
            Self::Normals(_normals) => Normals::scatter(intersection, scatter_index),
            Self::Metallic(metallic) => metallic.scatter(ray, intersection),
            Self::Refractive(refractive) => refractive.scatter(ray, intersection)
        }
    }
}
