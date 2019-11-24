use crate::math::Vec2;
use crate::math::Vec3;
use crate::math::Ray;

pub struct Camera
{
    position: Vec3,
    look_at: Option<Vec3>,
    up: Vec3,

    field_of_view: f32,
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
            look_at: None,
            up: Vec3::new(0.0, 1.0, 0.0),

            field_of_view: 90.0,
            source_size: Vec2::new(1.0, 1.0),
            target_size: Vec2::new(1024.0, 576.0),

            near_plane_corner: Vec3::new(0.0, 0.0, 0.0),
            near_plane_width: Vec3::new(0.0, 0.0, 0.0),
            near_plane_height: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_position(mut self, position: Vec3) -> Self
    {
        self.position = position;
        self
    }

    pub fn set_look_at(mut self, target: Option<Vec3>) -> Self
    {
        self.look_at = target;
        self
    }

    pub fn set_field_of_view(mut self, degrees: f32) -> Self
    {
        self.field_of_view = degrees;
        self
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

        corrected_size.x *= self.field_of_view / 90.0;
        corrected_size.y *= self.field_of_view / 90.0;

        let look_at = self.look_at.unwrap_or(self.position + Vec3::new(0.0, 0.0, -1.0));

        let w = (self.position - look_at).normalized();
        let u = self.up.cross(w).normalized();
        let v = w.cross(u);

        self.near_plane_corner = self.position - u * corrected_size.x * 0.5 - v * corrected_size.y * 0.5 - w;
        self.near_plane_width = u * corrected_size.x;
        self.near_plane_height = v * corrected_size.y;

        self
    }

    pub fn calculate_ray(&self, u: f32, v: f32) -> Ray
    {
        Ray::new(self.position, (self.near_plane_corner + self.near_plane_width * u + self.near_plane_height * v - self.position).normalized())
    }
}
