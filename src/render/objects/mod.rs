use super::math;
use super::materials;
use super::math::Ray;
use super::math::Intersection;
use super::materials::Material;

pub mod sphere;
pub use sphere::Sphere;

#[typetag::serde]
pub trait Object: Sync
{
    fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>;
    fn get_material(&self) -> &dyn Material;
}
