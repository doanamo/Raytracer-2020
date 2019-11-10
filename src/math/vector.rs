use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3
{
    pub fn new(x: f32, y: f32, z: f32) -> Vec3
    {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f32
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(self) -> Vec3
    {
        self / self.length()
    }

    pub fn dot(self, v: Vec3) -> f32
    {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl ops::Add<Vec3> for Vec3
{
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::Sub<Vec3> for Vec3
{
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl ops::Mul<f32> for Vec3
{
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3
    {
        Vec3
        {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl ops::Div<f32> for Vec3
{
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3
    {
        Vec3
        {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}
