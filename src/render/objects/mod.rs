use serde::{ Serialize, Deserialize };

use super::math;
use super::materials;
use super::math::Vec3;
use super::math::Ray;
use super::math::Intersection;
use super::math::Intersectable;
use super::materials::Material;

pub mod sphere;
pub use sphere::Sphere;

#[derive(Serialize, Deserialize)]
pub enum ObjectKind
{
    Sphere(Sphere)
}

#[derive(Serialize, Deserialize)]
pub struct Object
{
    #[serde(flatten)]
    kind: ObjectKind,

    #[serde(skip_serializing_if = "Vec3::is_zero", default)] 
    velocity: Vec3
}

impl Object
{
    pub fn new(kind: ObjectKind) -> Self
    {
        Self
        {
            kind,
            velocity: Vec3::zero()
        }
    }

    pub fn new_moving(kind: ObjectKind, velocity: Vec3) -> Self
    {
        Self
        {
            kind,
            velocity
        }
    }

    pub fn velocity(&self) -> Vec3
    {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec3)
    {
        self.velocity = velocity;
    }

    pub fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>
    {
        match &self.kind
        {
            ObjectKind::Sphere(sphere) =>
            {
                sphere.at_time(ray.time(), self.velocity()).intersect(ray, min_length, max_length)
            }
        }
    }

    pub fn get_material(&self) -> &Material
    {
        match &self.kind
        {
            ObjectKind::Sphere(sphere) => &sphere.material
        }
    }
}
