use crate::math::Color;
use crate::math::Vec3;
use crate::math::Ray;
use super::primitive::Intersection;

pub trait Material
{
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Ray, Color)>;
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

    pub fn from(albedo: Color) -> Self
    {
        Lambertian
        {
            albedo
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
    fn scatter(&self, _ray: &Ray, intersection: &Intersection) -> Option<(Ray, Color)>
    {
        let scatter_target = intersection.point + intersection.normal + Vec3::random_in_unit_sphere();
        let scattered_ray = Ray::new(intersection.point, (scatter_target - intersection.point).normalized());

        Some((scattered_ray, self.albedo))
    }
}

pub struct Metalic
{
    albedo: Color
}

impl Metalic
{
    pub fn new() -> Self
    {
        Metalic
        {
            albedo: Color::new(1.0, 1.0, 1.0, 1.0)
        }
    }

    pub fn from(albedo: Color) -> Self
    {
        Metalic
        {
            albedo
        }
    }

    pub fn set_albedo(mut self, color: Color) -> Self
    {
        self.albedo = color;
        self
    }
}

impl Material for Metalic
{
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Ray, Color)>
    {
        let reflected_dir = ray.get_direction().reflected(intersection.normal);
        let scattered_ray = Ray::new(intersection.point, reflected_dir);

        if scattered_ray.get_direction().dot(intersection.normal) > 0.0
        {
            Some((scattered_ray, self.albedo))
        }
        else
        {
            None
        }
    }
}
