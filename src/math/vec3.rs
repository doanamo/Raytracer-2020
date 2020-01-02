use std::cmp;
use std::ops;
use super::Vec2;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3
{
    pub fn new(x: f32, y: f32, z: f32) -> Self
    {
        Self
        {
            x,
            y,
            z
        }
    }

    pub fn forward() -> Self
    {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn right() -> Self
    {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn up() -> Self
    {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn random_direction() -> Self
    {
        let z = 2.0 * rand::random::<f32>() - 1.0;
        let planar = Vec2::random_direction() * (1.0 - z * z).sqrt();

        Self
        {
            x: planar.x,
            y: planar.y,
            z
        }
    }

    pub fn random_in_unit_sphere() -> Self
    {
        loop
        {
            let point = Self::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) * 2.0 - Self::new(1.0, 1.0, 1.0);

            if point.length_sqr() <= 1.0
            {
                return point;
            }
        }
    }

    pub fn random_in_unit_disc() -> Self
    {
        loop
        {
            let point = Self::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) * 2.0 - Self::new(1.0, 1.0, 0.0);

            if point.length_sqr() <= 1.0
            {
                return point;
            }
        }
    }

    pub fn dot(self, other: Self) -> f32
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self
    {
        Self
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

    pub fn normalized(self) -> Self
    {
        self / self.length()
    }

    pub fn reflected(self, normal: Self) -> Self
    {
        debug_assert!(normal.is_unit());
        self - normal * 2.0 * self.dot(normal)
    }

    pub fn refracted(self, normal: Self, eta: f32) -> Option<Self>
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

impl cmp::PartialEq for Vec3
{
    fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl ops::Add<Vec3> for Vec3
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        Self
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::Sub<Vec3> for Vec3
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
        Self
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl ops::Mul<f32> for Vec3
{
    type Output = Self;

    fn mul(self, other: f32) -> Self
    {
        Self
        {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl ops::Div<f32> for Vec3
{
    type Output = Self;

    fn div(self, other: f32) -> Self
    {
        Self
        {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn new()
    {
        assert_eq!(Vec3::new(3.0, 2.0, 1.0), Vec3::new(3.0, 2.0, 1.0));
        assert_eq!(Vec3::default(), Vec3::new(0.0, 0.0, 0.0));

        let vector = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);

        assert_eq!(Vec3::forward(), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(Vec3::right(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(Vec3::up(), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn random()
    {
        for _ in 0..100
        {
            let direction = Vec3::random_direction();
            assert!(direction.is_unit());
        }

        for _ in 0..100
        {
            let point = Vec3::random_in_unit_sphere();
            assert!(point.length_sqr() <= 1.0);
        }

        for _ in 0..100
        {
            let point = Vec3::random_in_unit_disc();
            assert!(point.length_sqr() <= 1.0);
            assert!(point.z == 0.0);
        }
    }

    #[test]
    fn calculate()
    {
        let vec_a = Vec3::new(1.0, 2.0, 3.0);
        let vec_b = Vec3::new(2.0, 4.0, 6.0);
  
        assert_eq!(vec_a.dot(vec_b), 28.0);
        assert_eq!(vec_a.cross(vec_b), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(vec_a.length(), 3.7416575);
        assert_eq!(vec_a.length_sqr(), 14.0);
        assert_eq!(vec_a.normalized(), Vec3::new(0.26726124, 0.5345225, 0.8017837));
        assert_eq!(vec_a.normalized(), vec_b.normalized());
        
        assert!((vec_b.length_sqr() - vec_b.length() * vec_b.length()).abs() < 0.00001);
        assert!(vec_b.length() == vec_b.length_sqr().sqrt());
        
        assert!(vec_a.normalized().is_unit());
        assert!(vec_b.normalized().is_unit());

        assert_eq!(vec_a, vec_a);
        assert_eq!(vec_a + vec_b, Vec3::new(3.0, 6.0, 9.0));
        assert_eq!(vec_a - vec_b, Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(vec_a * 4.0, Vec3::new(4.0, 8.0, 12.0));
        assert_eq!(vec_b / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn reflect()
    {
        let reflected = Vec3::new(-0.57735, -0.57735, -0.57735).reflected(Vec3::up());
        
        assert!(reflected.is_unit());
        assert_eq!(reflected, Vec3::new(-0.57735, -0.57735, 0.57735));
    }

    #[test]
    #[should_panic]
    fn reflect_bad_normal()
    {
        Vec3::forward().reflected(Vec3::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn refract()
    {
        // todo: Write unit tests for refraction.
        // Refraction computation needs to be understood better by me to write effective test.
        // I should create a simple refraction scene to observe how it works.
    }

    #[test]
    #[should_panic]
    fn refract_bad_vector()
    {
        Vec3::new(0.5, 0.5, 0.5).refracted(Vec3::forward(), 0.1);
    }

    #[test]
    #[should_panic]
    fn refract_bad_normal()
    {
        Vec3::forward().refracted(Vec3::new(0.5, 0.5, 0.5), 0.1);
    }
}
