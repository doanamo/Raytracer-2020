use serde::{ Serialize, Deserialize };
use super::math::Vec3;
use super::math::Vec4;
use super::math::Ray;
use super::math::Intersection;
use super::Material;

#[derive(Serialize, Deserialize)]
pub struct Diffuse
{
    albedo: Vec4
}

impl Default for Diffuse
{
    fn default() -> Self
    {
        Self
        {
            albedo: Vec4::new(0.5, 0.5, 0.5, 1.0)
        }
    }
}

impl Diffuse
{
    #[allow(clippy::new_ret_no_self)]
    pub fn new(albedo: Vec4) -> Material
    {
        Material::Diffuse(Self
        {
            albedo
        })
    }

    pub fn scatter(&self, ray: &Ray, intersection: &Intersection) -> (Option<Ray>, Vec4)
    {
        let scatter_target = intersection.point + intersection.normal + Vec3::random_in_unit_sphere();
        let scattered_ray = Ray::new(intersection.point, (scatter_target - intersection.point).normalized(), ray.time());

        (Some(scattered_ray), self.albedo)
    }
}
