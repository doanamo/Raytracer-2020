use super::vec2::Vec2;

#[cfg(target_feature = "sse2")]
pub use self::sse2::*;

#[cfg(not(target_feature = "sse2"))]
pub use self::scalar::*;

#[cfg(target_feature = "sse2")]
mod sse2
{
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use std::cmp;
    use std::ops;
    use std::fmt;

    macro_rules! _mm_shuffle
    {
        ($z:expr, $y:expr, $x:expr, $w:expr) =>
        {
            ($z << 6) | ($y << 4) | ($x << 2) | $w
        };
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Vec3(__m128);

    impl Default for Vec3
    {
        #[inline]
        fn default() -> Self
        {
            unsafe
            {
                Self(_mm_set1_ps(0.0))
            }
        }
    }

    impl Vec3
    {
        #[inline]
        pub fn new(x: f32, y: f32, z: f32) -> Self
        {
            unsafe
            {
                Self(_mm_set_ps(z, z, y, x))
            }
        }

        #[inline]
        pub fn zero() -> Self
        {
            unsafe
            {
                Self(_mm_set1_ps(0.0))
            }
        }

        #[inline]
        pub fn one() -> Self
        {
            unsafe
            {
                Self(_mm_set1_ps(1.0))
            }
        }

        #[inline]
        pub fn forward() -> Self
        {
            Self::new(0.0, 1.0, 0.0)
        }

        #[inline]
        pub fn right() -> Self
        {
            Self::new(1.0, 0.0, 0.0)
        }

        #[inline]
        pub fn up() -> Self
        {
            Self::new(0.0, 0.0, 1.0)
        }

        #[inline]
        pub fn set_x(&mut self, x: f32)
        {
            unsafe
            {
                self.0 = _mm_move_ss(self.0, _mm_set_ss(x));
            }
        }

        #[inline]
        pub fn set_y(&mut self, y: f32)
        {
            unsafe
            {
                let mut v = _mm_move_ss(self.0, _mm_set_ss(y));
                v = _mm_shuffle_ps(v, v, _mm_shuffle!(3, 2, 0, 0));
                self.0 = _mm_move_ss(v, self.0);
            }
        }

        #[inline]
        pub fn set_z(&mut self, z: f32)
        {
            unsafe
            {
                let mut v = _mm_move_ss(self.0, _mm_set_ss(z));
                v = _mm_shuffle_ps(v, v, _mm_shuffle!(3, 0, 1, 0));
                self.0 = _mm_move_ss(v, self.0);
            }
        }

        #[inline]
        pub fn get_x(self) -> f32
        {
            unsafe
            {
                _mm_cvtss_f32(self.0)
            }
        }

        #[inline]
        pub fn get_y(self) -> f32
        {
            unsafe
            {
                _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _mm_shuffle!(1, 1, 1, 1)))
            }
        }

        #[inline]
        pub fn get_z(self) -> f32
        {
            unsafe
            {
                _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _mm_shuffle!(2, 2, 2, 2)))
            }
        }

        #[inline]
        pub fn as_yzx(self) -> Self
        {
            unsafe
            {
                Self(_mm_shuffle_ps(self.0, self.0, _mm_shuffle!(0, 0, 2, 1)))
            }
        }

        #[inline]
        pub fn as_zxy(self) -> Self
        {
            unsafe
            {
                Self(_mm_shuffle_ps(self.0, self.0, _mm_shuffle!(1, 1, 0, 2)))
            }
        }

        #[inline]
        pub fn sum(self) -> f32
        {
            self.get_x() + self.get_y() + self.get_z()
        }

        #[inline]
        pub fn dot(self, other: Self) -> f32
        {
            (self * other).sum()
        }

        #[inline]
        pub fn cross(self, other: Self) -> Self
        {
            (self.as_zxy() * other - self * other.as_zxy()).as_zxy()
        }
    }

    impl cmp::PartialEq for Vec3
    {
        #[inline]
        fn eq(&self, other: &Self) -> bool
        {
            unsafe
            {
                let v = _mm_cmpeq_ps(self.0, other.0);
                _mm_movemask_ps(v) == 15
            }
        }
    }

    impl ops::Add for Vec3
    {
        type Output = Self;

        #[inline]
        fn add(self, other: Self) -> Self
        {
            unsafe
            {
                Self(_mm_add_ps(self.0, other.0))
            }
        }
    }

    impl ops::Sub for Vec3
    {
        type Output = Self;

        #[inline]
        fn sub(self, other: Self) -> Self
        {
            unsafe
            {
                Self(_mm_sub_ps(self.0, other.0))
            }
        }
    }

    impl ops::Mul for Vec3
    {
        type Output = Self;

        #[inline]
        fn mul(self, other: Self) -> Self
        {
            unsafe
            {
                Self(_mm_mul_ps(self.0, other.0))
            }
        }
    }

    impl ops::Div for Vec3
    {
        type Output = Self;

        #[inline]
        fn div(self, other: Self) -> Self
        {
            unsafe
            {
                Self(_mm_div_ps(self.0, other.0))
            }
        }
    }

    impl ops::Mul<f32> for Vec3
    {
        type Output = Self;

        #[inline]
        fn mul(self, other: f32) -> Self
        {
            unsafe
            {
                Self(_mm_mul_ps(self.0, _mm_set1_ps(other)))
            }
        }
    }

    impl ops::Div<f32> for Vec3
    {
        type Output = Self;

        #[inline]
        fn div(self, other: f32) -> Self
        {
            unsafe
            {
                Self(_mm_div_ps(self.0, _mm_set1_ps(other)))
            }
        }
    }

    impl fmt::Debug for Vec3
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "({}, {}, {})", self.get_x(), self.get_y(), self.get_z())
        }
    }
}

mod scalar
{
    use std::cmp;
    use std::ops;

    #[derive(Debug, Default, Copy, Clone)]
    pub struct Vec3(f32, f32, f32);

    impl Vec3
    {
        #[inline]
        pub fn new(x: f32, y: f32, z: f32) -> Self
        {
            Self(x, y, z)
        }

        #[inline]
        pub fn zero() -> Self
        {
            Self::new(0.0, 0.0, 0.0)
        }

        #[inline]
        pub fn one() -> Self
        {
            Self::new(1.0, 1.0, 1.0)
        }

        #[inline]
        pub fn forward() -> Self
        {
            Self::new(0.0, 1.0, 0.0)
        }

        #[inline]
        pub fn right() -> Self
        {
            Self::new(1.0, 0.0, 0.0)
        }

        #[inline]
        pub fn up() -> Self
        {
            Self::new(0.0, 0.0, 1.0)
        }

        #[inline]
        pub fn set_x(&mut self, x: f32)
        {
            self.0 = x;
        }

        #[inline]
        pub fn set_y(&mut self, y: f32)
        {
            self.1 = y;
        }

        #[inline]
        pub fn set_z(&mut self, z: f32)
        {
            self.2 = z;
        }

        #[inline]
        pub fn get_x(self) -> f32
        {
            self.0
        }

        #[inline]
        pub fn get_y(self) -> f32
        {
            self.1
        }

        #[inline]
        pub fn get_z(self) -> f32
        {
            self.2
        }

        #[inline]
        pub fn as_yzx(self) -> Self
        {
            Self(self.1, self.2, self.0)
        }

        #[inline]
        pub fn as_zxy(self) -> Self
        {
            Self(self.2, self.0, self.1)
        }

        #[inline]
        pub fn sum(self) -> f32
        {
            self.0 + self.1 + self.2
        }

        #[inline]
        pub fn dot(self, other: Self) -> f32
        {
            self.0 * other.0 +
            self.1 * other.1 +
            self.2 * other.2
        }

        #[inline]
        pub fn cross(self, other: Self) -> Self
        {
            Self
            (
                self.1 * other.2 - other.1 * self.2,
                self.2 * other.0 - other.2 * self.0,
                self.0 * other.1 - other.0 * self.1
            )
        }
    }

    impl cmp::PartialEq for Vec3
    {
        #[inline]
        fn eq(&self, other: &Self) -> bool
        {
            self.0 == other.0 &&
            self.1 == other.1 &&
            self.2 == other.2
        }
    }

    impl ops::Add for Vec3
    {
        type Output = Self;

        #[inline]
        fn add(self, other: Self) -> Self
        {
            Self
            (
                self.0 + other.0,
                self.1 + other.1,
                self.2 + other.2
            )
        }
    }

    impl ops::Sub for Vec3
    {
        type Output = Self;

        #[inline]
        fn sub(self, other: Self) -> Self
        {
            Self
            (
                self.0 - other.0,
                self.1 - other.1,
                self.2 - other.2
            )
        }
    }

    impl ops::Mul for Vec3
    {
        type Output = Self;

        #[inline]
        fn mul(self, other: Self) -> Self
        {
            Self
            (
                self.0 * other.0,
                self.1 * other.1,
                self.2 * other.2
            )
        }
    }

    impl ops::Div for Vec3
    {
        type Output = Self;

        #[inline]
        fn div(self, other: Self) -> Self
        {
            Self
            (
                self.0 / other.0,
                self.1 / other.1,
                self.2 / other.2
            )
        }
    }

    impl ops::Mul<f32> for Vec3
    {
        type Output = Self;

        #[inline]
        fn mul(self, other: f32) -> Self
        {
            Self
            (
                self.0 * other,
                self.1 * other,
                self.2 * other
            )
        }
    }

    impl ops::Div<f32> for Vec3
    {
        type Output = Self;

        #[inline]
        fn div(self, other: f32) -> Self
        {
            Self
            (
                self.0 / other,
                self.1 / other,
                self.2 / other
            )
        }
    }
}

macro_rules! vec3_shared
(
    ($module:ident) =>
    {
        impl $module::Vec3
        {
            #[inline]
            pub fn length_sqr(self) -> f32
            {
                self.dot(self)
            }

            #[inline]
            pub fn length(self) -> f32
            {
                self.length_sqr().sqrt()
            }

            #[inline]
            pub fn is_unit(self) -> bool
            {
                (self.length_sqr() - 1.0).abs() < 0.0001
            }

            #[inline]
            pub fn normalized(self) -> Self
            {
                self / self.length()
            }

            #[inline]
            pub fn reflected(self, normal: Self) -> Self
            {
                debug_assert!(normal.is_unit());
                self - normal * 2.0 * self.dot(normal)
            }

            #[inline]
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

            #[inline]
            pub fn random_direction() -> Self
            {
                let z = 2.0 * rand::random::<f32>() - 1.0;
                let planar = Vec2::random_direction() * (1.0 - z * z).sqrt();
                Self::new(planar.x, planar.y, z)
            }

            #[inline]
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

            #[inline]
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
        }
    }
);

#[cfg(target_feature = "sse2")]
vec3_shared!(sse2);
vec3_shared!(scalar);

#[cfg(test)]
mod tests
{
    pub use super::*;

    macro_rules! vec3_tests
    (
        ($module:ident) =>
        {
            mod $module
            {
                use crate::math::types::vec3::$module::Vec3;

                #[test]
                fn new()
                {
                    assert_eq!(Vec3::new(3.0, 2.0, 1.0), Vec3::new(3.0, 2.0, 1.0));
                    assert_eq!(Vec3::default(), Vec3::new(0.0, 0.0, 0.0));
                    assert_eq!(Vec3::zero(), Vec3::new(0.0, 0.0, 0.0));
                    assert_eq!(Vec3::one(), Vec3::new(1.0, 1.0, 1.0));

                    let mut vector = Vec3::new(1.0, 2.0, 3.0);
                    assert_eq!(vector.get_x(), 1.0);
                    assert_eq!(vector.get_y(), 2.0);
                    assert_eq!(vector.get_z(), 3.0);

                    vector.set_x(4.0);
                    vector.set_y(5.0);
                    vector.set_z(6.0);

                    assert_eq!(vector.get_x(), 4.0);
                    assert_eq!(vector.get_y(), 5.0);
                    assert_eq!(vector.get_z(), 6.0);

                    assert_eq!(vector.as_yzx(), Vec3::new(5.0, 6.0, 4.0));
                    assert_eq!(vector.as_zxy(), Vec3::new(6.0, 4.0, 5.0));

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
                        assert!(point.get_z() == 0.0);
                    }
                }

                #[test]
                fn calculate()
                {
                    let vec_a = Vec3::new(1.0, 2.0, 3.0);
                    let vec_b = Vec3::new(2.0, 4.0, 6.0);
            
                    assert_eq!(vec_a.sum(), 6.0);
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
                    assert_eq!(vec_a * vec_b, Vec3::new(2.0, 8.0, 18.0));
                    assert_eq!(vec_a / vec_b, Vec3::new(0.5, 0.5, 0.5));
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
        }
    );

    vec3_tests!(sse2);
    vec3_tests!(scalar);
}
