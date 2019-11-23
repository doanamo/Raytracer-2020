use crate::math::Color;
use crate::math::Vec3;
use crate::math::Ray;
use super::primitive::Intersection;

pub trait Material
{
    fn scatter(&self, ray: &Ray, intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Color);
}

pub struct Diffuse
{
    albedo: Color
}

impl Diffuse
{
    pub fn new() -> Self
    {
        Diffuse
        {
            albedo: Color::new(0.5, 0.5, 0.5, 1.0)
        }
    }

    pub fn from(albedo: Color) -> Self
    {
        Diffuse
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

impl Material for Diffuse
{
    fn scatter(&self, _ray: &Ray, intersection: &Intersection, _scatter_index: u16) -> (Option<Ray>, Color)
    {
        let scatter_target = intersection.point + intersection.normal + Vec3::random_in_unit_sphere();
        let scattered_ray = Ray::new(intersection.point, (scatter_target - intersection.point).normalized());

        (Some(scattered_ray), self.albedo)
    }
}

pub struct Metalic
{
    albedo: Color,
    roughness: f32
}

impl Metalic
{
    pub fn new() -> Self
    {
        Metalic
        {
            albedo: Color::new(1.0, 1.0, 1.0, 1.0),
            roughness: 0.0
        }
    }

    pub fn from(albedo: Color, roughness: f32) -> Self
    {
        Metalic
        {
            albedo,
            roughness
        }
    }

    pub fn set_albedo(mut self, color: Color) -> Self
    {
        self.albedo = color;
        self
    }

    pub fn set_roughness(mut self, roughness: f32) -> Self
    {
        self.roughness = roughness;
        self
    }
}

impl Material for Metalic
{
    fn scatter(&self, ray: &Ray, intersection: &Intersection, _scatter_index: u16) -> (Option<Ray>, Color)
    {
        let reflection_rougness = Vec3::random_in_unit_sphere() * self.roughness;
        let reflected_dir = (ray.get_direction().reflected(intersection.normal) + reflection_rougness).normalized();
        let scattered_ray = Ray::new(intersection.point, reflected_dir);

        (Some(scattered_ray), self.albedo)
    }
}
