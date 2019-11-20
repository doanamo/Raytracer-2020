use crate::math::Vec2;
use crate::math::Vec3;
use crate::math::Ray;

pub struct Camera
{
    position: Vec3,

    source_size: Vec2,
    target_size: Vec2,

    near_plane_corner: Vec3,
    near_plane_width: Vec3,
    near_plane_height: Vec3
}

impl Camera
{
    pub fn new() -> Self
    {
        Camera
        {
            position: Vec3::new(0.0, 0.0, 0.0),

            source_size: Vec2::new(2.0, 2.0),
            target_size: Vec2::new(1024.0, 576.0),

            near_plane_corner: Vec3::new(0.0, 0.0, 0.0),
            near_plane_width: Vec3::new(0.0, 0.0, 0.0),
            near_plane_height: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_source_size(mut self, width: f32, height: f32) -> Self
    {
        self.source_size = Vec2::new(width, height);
        self
    }

    pub fn set_target_size(mut self, width: usize, height: usize) -> Self
    {
        self.target_size = Vec2::new(width as f32, height as f32);
        self
    }

    pub fn build(mut self) -> Self
    {
        let mut corrected_size = self.source_size;

        let source_aspect_ratio = self.source_size.x / self.source_size.y;
        let target_aspect_ratio = self.target_size.x / self.target_size.y;

        if target_aspect_ratio > source_aspect_ratio
        {
            corrected_size.x *= target_aspect_ratio / source_aspect_ratio;
        }
        else
        {
            corrected_size.y *= source_aspect_ratio / target_aspect_ratio;
        }

        self.near_plane_corner = Vec3::new(corrected_size.x * -0.5, corrected_size.y * -0.5, -1.0);
        self.near_plane_width = Vec3::new(corrected_size.x, 0.0, 0.0);
        self.near_plane_height = Vec3::new(0.0, corrected_size.y, 0.0);

        self
    }

    pub fn calculate_ray(&self, u: f32, v: f32) -> Ray
    {
        Ray::new(self.position, (self.near_plane_corner + self.near_plane_width * u + self.near_plane_height * v).normalized())
    }
}
