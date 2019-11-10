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

    pub fn point_at(&self, alpha: f32) -> Vec3
    {
        self.origin + self.direction * alpha
    }

    pub fn intersect_sphere(&self, center: Vec3, radius: f32) -> f32
    {
        let oc = self.origin - center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0
        {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
        else
        {
            discriminant
        }
    }
}
