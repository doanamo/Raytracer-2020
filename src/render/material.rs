use crate::math::Color;
use crate::math::Vec3;
use crate::math::Ray;
use super::primitive::Intersection;

pub trait Material
{
    fn scatter(&self, intersection: &Intersection) -> Option<(Ray, Color)>;
}

pub struct Lambertian
{
    albedo: Color
}

impl Lambertian
{
    pub fn new() -> Self
    {
        Lambertian
        {
            albedo: Color::new(0.5, 0.5, 0.5, 1.0)
        }
    }

    pub fn set_albedo(mut self, color: Color) -> Self
    {
        self.albedo = color;
        self
    }
}

impl Material for Lambertian
{
    fn scatter(&self, intersection: &Intersection) -> Option<(Ray, Color)>
    {
        let bounce_target = intersection.point + intersection.normal + Vec3::random_in_unit_sphere();
        let bounce_ray = Ray::new(intersection.point, (bounce_target - intersection.point).normalized());

        Some((bounce_ray, self.albedo))
    }
}
