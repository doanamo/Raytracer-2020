use serde::{ Serialize, Deserialize };
use super::math::Color;
use super::math::Vec3;
use super::math::Ray;
use super::math::Intersection;
use super::Material;

#[derive(Serialize, Deserialize)]
pub struct Diffuse
{
    albedo: Color
}

impl Default for Diffuse
{
    fn default() -> Self
    {
        Self
        {
            albedo: Color::new(0.5, 0.5, 0.5, 1.0)
        }
    }
}

impl Diffuse
{
    #[allow(clippy::new_ret_no_self)]
    pub fn new(albedo: Color) -> Material
    {
        Material::Diffuse(Self
        {
            albedo
        })
    }

    pub fn scatter(&self, intersection: &Intersection) -> (Option<Ray>, Color)
    {
        let scatter_target = intersection.point + intersection.normal + Vec3::random_in_unit_sphere();
        let scattered_ray = Ray::new(intersection.point, (scatter_target - intersection.point).normalized());

        (Some(scattered_ray), self.albedo)
    }
}
