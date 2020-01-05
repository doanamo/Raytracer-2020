use serde::{ Serialize, Deserialize };
use super::math::Color;
use super::math::Ray;
use super::math::Intersection;
use super::Material;

#[derive(Serialize, Deserialize)]
pub struct Refractive
{
    albedo: Color,
    refractive_index: f32
}

impl Default for Refractive
{
    fn default() -> Self
    {
        Self
        {
            albedo: Color::new(1.0, 1.0, 1.0, 1.0),
            refractive_index: 0.0
        }
    }
}

impl Refractive
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn from(albedo: Color, refractive_index: f32) -> Self
    {
        Self
        {
            albedo,
            refractive_index
        }
    }
}

#[typetag::serde]
impl Material for Refractive
{
    fn scatter(&self, ray: &Ray, intersection: &Intersection, _scatter_index: u16) -> (Option<Ray>, Color)
    {
        let outward_normal;
        let cosine;
        let eta;

        let schlick = |cosine: f32, refractive_index: f32|
        {
            let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
            r0 = r0 * r0;
            r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
        };

        if ray.direction.dot(intersection.normal) > 0.0
        {
            outward_normal = intersection.normal * -1.0;
            eta = self.refractive_index;
            cosine = self.refractive_index * ray.direction.dot(intersection.normal) / ray.direction.length();
        }
        else
        {
            outward_normal = intersection.normal;
            eta = 1.0 / self.refractive_index;
            cosine = -1.0 * ray.direction.dot(intersection.normal) / ray.direction.length();
        }

        if let Some(refracted) = ray.direction.refracted(outward_normal, eta)
        {
            let reflection_propability = schlick(cosine, self.refractive_index);

            if rand::random::<f32>() >= reflection_propability
            {
                return (Some(Ray::new(intersection.point, refracted)), self.albedo);
            }
        }

        let reflected = ray.direction.reflected(intersection.normal);
        (Some(Ray::new(intersection.point, reflected)), self.albedo)
    }
}
