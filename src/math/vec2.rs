use std::cmp;
use std::ops;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec2
{
    pub x: f32,
    pub y: f32
}

impl Vec2
{
    pub fn new(x: f32, y: f32) -> Vec2
    {
        Vec2
        {
            x,
            y
        }
    }

    pub fn random_direction() -> Vec2
    {
        let azimuth = rand::random::<f32>() * 2.0 * std::f32::consts::PI;

        Vec2
        {
            x: azimuth.cos(),
            y: azimuth.sin()
        }
    }
}

impl cmp::PartialEq for Vec2
{
    fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x &&
        self.y == other.y
    }
}

impl ops::Add<Vec2> for Vec2
{
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl ops::Sub<Vec2> for Vec2
{
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl ops::Mul<f32> for Vec2
{
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2
    {
        Vec2
        {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl ops::Div<f32> for Vec2
{
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2
    {
        Vec2
        {
            x: self.x / other,
            y: self.y / other
        }
    }
}
