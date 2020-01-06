use serde::{ Serialize, Deserialize };

use super::math;
use super::math::Ray;
use super::math::Color;
use super::math::Intersection;

pub mod diffuse;
pub use diffuse::Diffuse;
pub mod normals;
pub use normals::Normals;
pub mod metalic;
pub use metalic::Metalic;
pub mod refractive;
pub use refractive::Refractive;

#[derive(Serialize, Deserialize)]
pub enum Material
{
    Diffuse(Diffuse),
    Normals(Normals),
    Metalic(Metalic),
    Refractive(Refractive)
}

impl Material
{
    pub fn scatter(&self, ray: &Ray, intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Color)
    {
        match &self
        {
            Self::Diffuse(diffuse) => diffuse.scatter(intersection),
            Self::Normals(_normals) => Normals::scatter(intersection, scatter_index),
            Self::Metalic(metalic) => metalic.scatter(ray, intersection),
            Self::Refractive(refractive) => refractive.scatter(ray, intersection)
        }
    }
}
