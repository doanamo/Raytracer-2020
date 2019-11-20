use crate::math::Vec3;

pub struct Ray
{
    origin: Vec3,
    direction: Vec3
}

impl Ray
{
    pub fn new(origin: Vec3, direction: Vec3) -> Ray
    {
        debug_assert!(direction.is_unit());

        Ray
        {
            origin,
            direction,
        }
    }

    pub fn get_origin(&self) -> Vec3
    {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3
    {
        self.direction
    }

    pub fn point_at(&self, length: f32) -> Vec3
    {
        self.origin + self.direction * length
    }
}
