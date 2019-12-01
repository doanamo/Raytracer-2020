use std::ops;

#[derive(Default, Copy, Clone)]
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

#[derive(Default, Copy, Clone)]
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

    pub fn random_in_unit_disc() -> Vec3
    {
        let mut point;

        loop
        {
            point = Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);

            if point.length_sqr() <= 1.0
            {
                break;
            }
        }

        point
    }

    pub fn dot(self, other: Vec3) -> f32
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y
        }
    }

    pub fn length_sqr(self) -> f32
    {
        self.dot(self)
    }

    pub fn length(self) -> f32
    {
        self.length_sqr().sqrt()
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

    pub fn refracted(self, normal: Vec3, eta: f32) -> Option<Vec3>
    {
        debug_assert!(self.is_unit());
        debug_assert!(normal.is_unit());

        let dot = self.dot(normal);
        let discriminant = 1.0 - eta * eta * (1.0 - dot * dot);

        if discriminant > 0.0
        {
            let refracted = (self - normal * dot) * eta - normal * discriminant.sqrt();

            Some(refracted)
        }
        else
        {
            None
        }
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
