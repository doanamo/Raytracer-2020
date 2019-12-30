use serde::{ Serialize, Deserialize };
use crate::math::Color;
use crate::math::Vec3;
use crate::math::Ray;
use super::primitive::Intersection;

#[typetag::serde]
pub trait Material: Sync
{
    fn scatter(&self, ray: &Ray, intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Color);
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Normals
{
}

impl Normals
{
    pub fn new() -> Self
    {
        Normals
        {
        }
    }
}

#[typetag::serde]
impl Material for Normals
{
    fn scatter(&self, _ray: &Ray, intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Color)
    {
        if scatter_index == 0
        {
            let normal_color = Color::new(intersection.normal.x + 1.0, intersection.normal.y + 1.0, intersection.normal.z + 1.0, 1.0).mul_rgb(0.5);

            (None, normal_color)
        }
        else
        {
            panic!("Did not expect debug material for normals to scatter!");
        }
    }
}

#[derive(serde::Serialize, Deserialize)]
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

#[typetag::serde]
impl Material for Metalic
{
    fn scatter(&self, ray: &Ray, intersection: &Intersection, _scatter_index: u16) -> (Option<Ray>, Color)
    {
        let reflection_rougness = Vec3::random_in_unit_sphere() * self.roughness;
        let reflected_dir = (ray.direction.reflected(intersection.normal) + reflection_rougness).normalized();
        let scattered_ray = Ray::new(intersection.point, reflected_dir);

        (Some(scattered_ray), self.albedo)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Refractive
{
    albedo: Color,
    refractive_index: f32
}

impl Refractive
{
    pub fn new() -> Self
    {
        Refractive
        {
            albedo: Color::new(1.0, 1.0, 1.0, 1.0),
            refractive_index: 0.0
        }
    }

    pub fn from(albedo: Color, refractive_index: f32) -> Self
    {
        Refractive
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

            return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0); 
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
            let reflection_probaility = schlick(cosine, self.refractive_index);

            if rand::random::<f32>() >= reflection_probaility
            {
                return (Some(Ray::new(intersection.point, refracted)), self.albedo);
            }
        }

        let reflected = ray.direction.reflected(intersection.normal);
        (Some(Ray::new(intersection.point, reflected)), self.albedo)
    }
}
