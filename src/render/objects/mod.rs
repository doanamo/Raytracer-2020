use serde::{ Serialize, Deserialize };

use super::math;
use super::materials;
use super::math::Ray;
use super::math::Intersection;
use super::materials::Material;

pub mod sphere;
pub use sphere::Sphere;

#[derive(Serialize, Deserialize)]
pub enum Object
{
    Sphere(Sphere)
}

impl Object
{
    pub fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>
    {
        match &self
        {
            Self::Sphere(sphere) => sphere.intersect(ray, min_length, max_length)
        }
    }

    pub fn get_material(&self) -> &Material
    {
        match &self
        {
            Self::Sphere(sphere) => sphere.get_material()
        }
    }
}
