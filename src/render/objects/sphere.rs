use serde::{ Serialize, Deserialize };
use super::math::Vec3;
use super::math::Ray;
use super::math::Intersection;
use super::math::geometry;
use super::materials::Material;
use super::Object;

#[derive(Serialize, Deserialize)]
pub struct Sphere
{
    #[serde(flatten)]
    pub shape: geometry::Sphere,
    pub material: Box<dyn Material + Sync>
}

impl Sphere
{
    pub fn new<MaterialType: Material + 'static>(center: Vec3, radius: f32, material: MaterialType) -> Self
    {
        Self
        {
            shape: geometry::Sphere
            {
                center,
                radius
            },
            material: Box::new(material)
        }
    }
}

#[typetag::serde]
impl Object for Sphere
{
    fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>
    {
        if let Some(intersection) = ray.intersect(&self.shape)
        {
            if min_length < intersection.length && intersection.length < max_length
            {
                return Some(intersection);
            }
        }

        None
    }

    fn get_material(&self) -> &dyn Material
    {
        self.material.as_ref()
    }
}
