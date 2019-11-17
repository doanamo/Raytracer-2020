use crate::math::Vec3;

pub struct Ray
{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray
{
    pub fn new(origin: Vec3, direction: Vec3) -> Ray
    {
        Ray
        {
            origin,
            direction,
        }
    }

    pub fn point_at(&self, length: f32) -> Vec3
    {
        self.origin + self.direction * length
    }
}
