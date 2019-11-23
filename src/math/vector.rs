use std::ops;

#[derive(Copy, Clone)]
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
        Vec3
        {
            x,
            y,
            z
        }
    }

    pub fn random_direction() -> Vec3
    {
        let z = 2.0 * rand::random::<f32>() - 1.0;
        let planar = Vec2::random_direction() * (1.0 - z * z).sqrt();

        Vec3
        {
            x: planar.x,
            y: planar.y,
            z: z
        }
    }

    pub fn random_in_unit_sphere() -> Vec3
    {
        let mut point;

        loop
        {
            point = Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);

            if point.length_sqr() < 1.0
            {
                break;
            }
        }

        point
    }

    pub fn length_sqr(self) -> f32
    {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn length(self) -> f32
    {
        self.length_sqr().sqrt()
    }

    pub fn dot(self, v: Vec3) -> f32
    {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn normalized(self) -> Vec3
    {
        self / self.length()
    }

    pub fn reflected(self, normal: Vec3) -> Vec3
    {
        debug_assert!(normal.is_unit());
        self - normal * 2.0 * self.dot(normal)
    }

    pub fn is_unit(self) -> bool
    {
        (self.length_sqr() - 1.0).abs() < 0.0001
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
