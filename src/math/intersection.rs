use super::types::vec3::Vec3;
use super::types::ray::Ray;

pub struct Intersection
{
    pub point: Vec3,
    pub normal: Vec3,
    pub length: f32
}

pub trait Intersectable
{
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
