use serde::{ Serialize, Deserialize };
use super::math::Color;
use super::math::Vec3;
use super::math::Ray;
use super::math::Intersection;
use super::Material;

#[derive(Serialize, Deserialize)]
pub struct Metalic
{
    albedo: Color,
    roughness: f32
}

impl Default for Metalic
{
    fn default() -> Self
    {
        Self
        {
            albedo: Color::new(1.0, 1.0, 1.0, 1.0),
            roughness: 0.0
        }
    }
}

impl Metalic
{
    pub fn new(albedo: Color, roughness: f32) -> Material
    {
        Material::Metalic(Self
        {
            albedo,
            roughness
        })
    }

    pub fn scatter(&self, ray: &Ray, intersection: &Intersection, _scatter_index: u16) -> (Option<Ray>, Color)
    {
        let reflection_rougness = Vec3::random_in_unit_sphere() * self.roughness;
        let reflected_dir = (ray.direction.reflected(intersection.normal) + reflection_rougness).normalized();
        let scattered_ray = Ray::new(intersection.point, reflected_dir);

        (Some(scattered_ray), self.albedo)
    }
}
