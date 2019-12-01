use serde::{ Serialize, Deserialize };
use crate::math::Vec3;
use crate::math::Ray;
use super::material::Material;

pub struct Intersection<'a>
{
    pub point: Vec3,
    pub normal: Vec3,
    pub length: f32,
    pub material: &'a dyn Material
}

#[typetag::serde]
pub trait Primitive
{
    fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>;
}

#[derive(Serialize, Deserialize)]
pub struct Sphere
{
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>
}

impl Sphere
{
    pub fn new<MaterialType: Material + 'static>(center: Vec3, radius: f32, material: MaterialType) -> Self
    {
        Sphere
        {
            center,
            radius,

            material: Box::new(material)
        }
    }
}

#[typetag::serde]
impl Primitive for Sphere
{
    fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>
    {
        debug_assert!(ray.get_direction().is_unit());

        let oc = ray.get_origin() - self.center;
        let a = ray.get_direction().dot(ray.get_direction());
        let b = 2.0 * oc.dot(ray.get_direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0
        {
            let r1 = (-b - discriminant.sqrt()) / (2.0 * a);

            if min_length < r1 && r1 < max_length
            {
                let intersection_point = ray.point_at(r1);

                return Some(Intersection
                {
                    point: intersection_point,
                    normal: (intersection_point - self.center) / self.radius,
                    length: r1,
                    material: self.material.as_ref()
                });
            }

            let r2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if min_length < r2 && r2 < max_length
            {
                let intersection_point = ray.point_at(r2);

                return Some(Intersection
                {
                    point: intersection_point,
                    normal: (intersection_point - self.center)  / self.radius,
                    length: r2,
                    material: self.material.as_ref()
                });
            }
        }

        None
    }
}
