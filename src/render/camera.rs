use serde::{ Serialize, Deserialize };
use crate::math::Vec3;
use crate::math::Ray;

#[derive(Debug)]
pub enum InvalidParameter
{
    FieldOfView,
    ApertureRadius,
    ShutterTime
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Parameters
{
    pub origin: Vec3,
    pub up_direction: Vec3,
    pub look_at: Option<Vec3>,

    pub field_of_view: f32,
    pub focus_distance: f32,
    pub aperture_radius: f32,
    pub shutter_open_time: f32,
    pub shutter_close_time: f32
}

impl Default for Parameters
{
    fn default() -> Self
    {
        Self
        {
            origin: Vec3::new(0.0, 0.0, 0.0),
            up_direction: Vec3::new(0.0, 0.0, 1.0),
            look_at: None,

            field_of_view: 90.0,
            focus_distance: 1.0,
            aperture_radius: 0.0,
            shutter_open_time: 0.0,
            shutter_close_time: 0.0
        }
    }
}

impl Parameters
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn set_origin(mut self, position: Vec3) -> Self
    {
        self.origin = position;
        self
    }
    
    pub fn set_up_direction(mut self, normal: Vec3) -> Self
    {
        self.up_direction = normal;
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

    pub fn set_focus_distance(mut self, distance: f32) -> Self
    {
        self.focus_distance = distance;
        self
    }

    pub fn set_focus_on_look_at(mut self, modifier: f32) -> Self
    {
        if let Some(look_at) = self.look_at
        {
            self.focus_distance = (self.origin - look_at).length() + modifier;
        }

        self
    }

    pub fn set_aperture_size(mut self, radius: f32) -> Self
    {
        self.aperture_radius = radius;
        self
    }

    pub fn set_shutter_open_time(mut self, time: f32) -> Self
    {
        self.shutter_open_time = time;
        self
    }

    pub fn set_shutter_close_time(mut self, time: f32) -> Self
    {
        self.shutter_close_time = time;
        self
    }

    pub fn build(&self, aspect_ratio: f32) -> Result<Compiled, InvalidParameter>
    {
        debug_assert!(self.up_direction.is_unit());

        if self.field_of_view <= 0.0
        {
            return Result::Err(InvalidParameter::FieldOfView);
        }

        if self.aperture_radius < 0.0
        {
            return Result::Err(InvalidParameter::ApertureRadius);
        }

        if self.shutter_close_time < self.shutter_open_time
        {
            return Result::Err(InvalidParameter::ShutterTime)
        }

        let half_height = (self.field_of_view * std::f32::consts::PI / 180.0 / 2.0).tan();
        let half_width = half_height * aspect_ratio;

        let look_at = self.look_at.unwrap_or(self.origin + Vec3::new(0.0, 1.0, 0.0));

        let forward_direction = (look_at - self.origin).normalized();
        let right_cross_product = forward_direction.cross(self.up_direction);

        let right_direction = if right_cross_product != Vec3::zero()
        {
            right_cross_product.normalized()
        }
        else
        {
            Vec3::right()
        };

        let up_direction = right_direction.cross(forward_direction);

        let near_plane_left_offset = right_direction * half_width * self.focus_distance;
        let near_plane_bottom_offset = up_direction * half_height * self.focus_distance;

        let near_plane_corner = self.origin + forward_direction * self.focus_distance - near_plane_left_offset - near_plane_bottom_offset;
        let near_plane_width = right_direction * half_width * 2.0 * self.focus_distance;
        let near_plane_height = up_direction * half_height * 2.0 * self.focus_distance;

        Ok(Compiled
        {
            origin: self.origin,

            aperture_radius: self.aperture_radius,
            shutter_open_time: self.shutter_open_time,
            shutter_close_time: self.shutter_close_time,

            near_plane_corner,
            near_plane_width,
            near_plane_height,

            right_direction,
            up_direction
        })
    }
}

pub struct Compiled
{
    origin: Vec3,

    aperture_radius: f32,
    shutter_open_time: f32,
    shutter_close_time: f32,

    near_plane_corner: Vec3,
    near_plane_width: Vec3,
    near_plane_height: Vec3,

    right_direction: Vec3,
    up_direction: Vec3
}

impl Compiled
{
    pub fn calculate_ray(&self, u: f32, v: f32) -> Ray
    {
        let random = Vec3::random_in_unit_disc() * self.aperture_radius;
        let offset = self.right_direction * random.get_x() + self.up_direction * random.get_y();

        let origin = self.origin + offset;
        let direction = self.near_plane_corner + self.near_plane_width * u + self.near_plane_height * v - origin;
        let time = self.shutter_open_time + rand::random::<f32>() * (self.shutter_close_time - self.shutter_open_time);

        Ray::new(origin, direction.normalized(), time)
    }
}
