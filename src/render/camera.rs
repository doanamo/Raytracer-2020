use serde::{ Serialize, Deserialize };
use crate::math::Vec3;
use crate::math::Ray;

#[derive(Serialize, Deserialize)]
pub struct Camera
{
    origin: Vec3,
    look_at: Option<Vec3>,

    #[serde(skip)]
    aspect_ratio: f32,
    field_of_view: f32,

    focus_distance: f32,
    aperture_radius: f32,

    #[serde(skip)]
    near_plane_corner: Vec3,
    #[serde(skip)]
    near_plane_width: Vec3,
    #[serde(skip)]
    near_plane_height: Vec3,

    #[serde(skip)]
    forward_dir: Vec3,
    #[serde(skip)]
    right_dir: Vec3,
    up_dir: Vec3,
}

impl Default for Camera
{
    fn default() -> Self
    {
        Camera
        {
            origin: Vec3::new(0.0, 0.0, 0.0),
            look_at: None,

            aspect_ratio: 1.0,
            field_of_view: 90.0,

            focus_distance: 1.0,
            aperture_radius: 0.0,

            near_plane_corner: Vec3::new(0.0, 0.0, 0.0),
            near_plane_width: Vec3::new(0.0, 0.0, 0.0),
            near_plane_height: Vec3::new(0.0, 0.0, 0.0),

            forward_dir: Vec3::new(0.0, 1.0, 0.0),
            right_dir: Vec3::new(1.0, 0.0, 0.0),
            up_dir: Vec3::new(0.0, 0.0, 1.0)
        }
    }
}

impl Camera
{
    pub fn new() -> Self
    {
        Camera::default()
    }

    pub fn set_origin(mut self, position: Vec3) -> Self
    {
        self.origin = position;
        self
    }

    pub fn set_look_at(mut self, target: Option<Vec3>) -> Self
    {
        self.look_at = target;
        self
    }

    pub fn set_up_direction(mut self, normal: Vec3) -> Self
    {
        debug_assert!(normal.is_unit());
        self.up_dir = normal;
        self
    }

    pub fn set_aspect_ratio(mut self, ratio: f32) -> Self
    {
        debug_assert!(ratio != 0.0);
        self.aspect_ratio = ratio;
        self
    }

    pub fn set_aspect_ratio_from_size(mut self, width: usize, height: usize) -> Self
    {
        debug_assert!(width != 0);
        debug_assert!(height != 0);
        self.aspect_ratio = width as f32 / height as f32;
        self
    }

    pub fn set_field_of_view(mut self, degrees: f32) -> Self
    {
        debug_assert!(degrees != 0.0);
        self.field_of_view = degrees;
        self
    }

    pub fn set_focus_distance(mut self, distance: f32) -> Self
    {
        self.focus_distance = distance;
        self
    }

    pub fn set_aperture_size(mut self, size: f32) -> Self
    {
        debug_assert!(size >= 0.0);
        self.aperture_radius = size;
        self
    }

    pub fn build(mut self) -> Self
    {
        let half_height = (self.field_of_view * std::f32::consts::PI / 180.0 / 2.0).tan();
        let half_width = half_height * self.aspect_ratio;

        let look_at = self.look_at.unwrap_or(self.origin + Vec3::new(0.0, 1.0, 0.0));

        self.forward_dir = (look_at - self.origin).normalized();
        self.right_dir = self.forward_dir.cross(self.up_dir).normalized();
        self.up_dir = self.right_dir.cross(self.forward_dir);

        let near_plane_left_offset = self.right_dir * half_width * self.focus_distance;
        let near_plane_top_offset = self.up_dir * half_height * self.focus_distance;

        self.near_plane_corner = self.origin + self.forward_dir * self.focus_distance - near_plane_left_offset - near_plane_top_offset;
        self.near_plane_width = self.right_dir * half_width * 2.0 * self.focus_distance;
        self.near_plane_height = self.up_dir * half_height * 2.0 * self.focus_distance;

        self
    }

    pub fn calculate_ray(&self, u: f32, v: f32) -> Ray
    {
        let random = Vec3::random_in_unit_disc() * self.aperture_radius;
        let offset = self.right_dir * random.x + self.up_dir * random.y;

        let origin = self.origin + offset;
        let direction = self.near_plane_corner + self.near_plane_width * u + self.near_plane_height * v - self.origin - offset;

        Ray::new(origin, direction.normalized())
    }
}
