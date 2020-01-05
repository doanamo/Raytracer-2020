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
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn from(albedo: Color) -> Self
    {
        Self
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

#[typetag::serde]
impl Material for Diffuse
{
    fn scatter(&self, _ray: &Ray, intersection: &Intersection, _scatter_index: u16) -> (Option<Ray>, Color)
    {
        let scatter_target = intersection.point + intersection.normal + Vec3::random_in_unit_sphere();
        let scattered_ray = Ray::new(intersection.point, (scatter_target - intersection.point).normalized());

        (Some(scattered_ray), self.albedo)
    }
}
