use serde::{ Serialize, Deserialize };
use super::math::Vec3;
use super::math::Vec4;
use super::math::Ray;
use super::math::Intersection;
use super::Material;

#[derive(Serialize, Deserialize)]
pub struct Metallic
{
    albedo: Vec4,
    roughness: f32
}

impl Default for Metallic
{
    fn default() -> Self
    {
        Self
        {
            albedo: Vec4::new(1.0, 1.0, 1.0, 1.0),
            roughness: 0.0
        }
    }
}

impl Metallic
{
    #[allow(clippy::new_ret_no_self)]
    pub fn new(albedo: Vec4, roughness: f32) -> Material
    {
        Material::Metallic(Self
        {
            albedo,
            roughness
        })
    }

    pub fn scatter(&self, ray: &Ray, intersection: &Intersection) -> (Option<Ray>, Vec4)
    {
        let reflection_rougness = Vec3::random_in_unit_sphere() * self.roughness;
        let reflected_dir = (ray.direction().reflected(intersection.normal) + reflection_rougness).normalized();
        let scattered_ray = Ray::new(intersection.point, reflected_dir, ray.time());

        (Some(scattered_ray), self.albedo)
    }
}
