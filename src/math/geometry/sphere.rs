use serde::{ Serialize, Deserialize };
use super::types::vec3::Vec3;
use super::types::ray::Ray;
use super::intersection::Intersectable;
use super::intersection::Intersection;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Sphere
{
    pub center: Vec3,
    pub radius: f32
}

impl Sphere
{
    pub fn new(center: Vec3, radius: f32) -> Self
    {
        Self
        {
            center,
            radius
        }
    }
}

impl Intersectable for Sphere
{
    fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>
    {
        // Implementation based on: http://viclw17.github.io/2018/07/16/raytracing-ray-sphere-intersection/
        // See comment from "T Jank" that explains and fixes a bug in the original implementation.
        // Quadratic functions refresher: https://en.wikipedia.org/wiki/Quadratic_function

        debug_assert!(ray.direction().is_unit());

        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0
        {
            let discriminant_sqrt = discriminant.sqrt();

            {
                let r1 = (-b - discriminant_sqrt) / (2.0 * a);
                
                if min_length < r1 && r1 < max_length
                {
                    let intersection_point = ray.point_at(r1);

                    return Some(Intersection
                    {
                        point: intersection_point,
                        normal: (intersection_point - self.center) / self.radius,
                        length: r1
                    });
                }
            }

            {
                let r2 = (-b + discriminant_sqrt) / (2.0 * a);

                if min_length < r2 && r2 < max_length
                {
                    let intersection_point = ray.point_at(r2);

                    return Some(Intersection
                    {
                        point: intersection_point,
                        normal: (intersection_point - self.center) / self.radius,
                        length: r2
                    });
                }
            }
        }

        None
    }
}
