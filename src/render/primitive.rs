use crate::math::Vec3;
use crate::math::Ray;

pub struct Intersection
{
    pub point: Vec3,
    pub normal: Vec3,
    pub length: f32,
}

pub trait Primitive
{
    fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>;
}
