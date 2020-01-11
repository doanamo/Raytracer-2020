use serde::{ Serialize, Deserialize };
use crate::math::Ray;
use crate::math::Intersection;
use super::camera;
use super::objects::Object;

#[derive(Default, Serialize, Deserialize)]
pub struct Scene
{
    pub camera: camera::Parameters,
    objects: Vec<Object>
}

impl Scene
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn set_camera(mut self, camera: camera::Parameters) -> Self
    {
        self.camera = camera;
        self
    }

    pub fn add_object(mut self, object: Object) -> Self
    {
        self.objects.push(object);
        self
    }

    pub fn intersect(&self, ray: &Ray, min_length: f32, max_length: f32) -> Option<(Intersection, &Object)>
    {
        let mut closest_intersection: Option<(Intersection, &Object)> = None;
        let mut closest_length = max_length;

        for object in &self.objects
        {
            if let Some(intersection) = object.intersect(ray, min_length, closest_length)
            {
                debug_assert!(intersection.length <= closest_length);

                closest_length = intersection.length;
                closest_intersection = Some((intersection, &object));
            }
        }

        closest_intersection
    }
}
