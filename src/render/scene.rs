use crate::math::Ray;
use super::primitive::Primitive;
use super::primitive::Intersection;

pub struct Scene
{
    primitives: Vec<Box<dyn Primitive>>
}

impl Scene
{
    pub fn new() -> Self
    {
        Scene
        {
            primitives: Vec::new()
        }
    }

    pub fn add_primitive(mut self, primitive: Box<dyn Primitive>) -> Self
    {
        self.primitives.push(primitive);
        self
    }

    pub fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<Intersection>
    {
        let mut closest_intersection: Option<Intersection> = None;
        let mut closest_length = max_length;

        for primitive in self.primitives.iter()
        {
            if let Some(intersection) = primitive.intersect(ray, min_length, closest_length)
            {
                closest_length = intersection.length;
                closest_intersection = Some(intersection);
            }
        }

        closest_intersection
    }
}
