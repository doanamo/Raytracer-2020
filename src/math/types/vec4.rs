pub use super::vec3;

#[cfg(target_feature = "sse2")]
pub use self::sse2::*;

#[cfg(not(target_feature = "sse2"))]
pub use self::scalar::*;

pub mod sse2
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
    pub struct Vec4(__m128);

    impl Default for Vec4
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

    impl Vec4
    {
        #[inline]
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self
        {
            unsafe
            {
                Self(_mm_set_ps(w, z, y, x))
            }
        }

        #[inline]
        pub fn zero() -> Self
        {
            Self::new(0.0, 0.0, 0.0, 0.0)
        }

        #[inline]
        pub fn one() -> Self
        {
            Self::new(1.0, 1.0, 1.0, 1.0)
        }

        #[inline]
        pub fn black() -> Self
        {
            Self::new(0.0, 0.0, 0.0, 1.0)
        }

        #[inline]
        pub fn white() -> Self
        {
            Self::new(1.0, 1.0, 1.0, 1.0)
        }

        #[inline]
        pub fn red() -> Self
        {
            Self::new(1.0, 0.0, 0.0, 1.0)
        }

        #[inline]
        pub fn green() -> Self
        {
            Self::new(0.0, 1.0, 0.0, 1.0)
        }

        #[inline]
        pub fn blue() -> Self
        {
            Self::new(0.0, 0.0, 1.0, 1.0)
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
        pub fn set_w(&mut self, w: f32)
        {
            unsafe
            {
                let mut v = _mm_move_ss(self.0, _mm_set_ss(w));
                v = _mm_shuffle_ps(v, v, _mm_shuffle!(0, 2, 1, 0));
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
        pub fn get_w(self) -> f32
        {
            unsafe
            {
                _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _mm_shuffle!(3, 3, 3, 3)))
            }
        }

        #[inline]
        pub fn clamped(self) -> Self
        {
            unsafe
            {
                Self(_mm_min_ps(_mm_max_ps(self.0, _mm_set1_ps(0.0)), _mm_set1_ps(1.0)))
            }
        }
    }

    impl cmp::PartialEq for Vec4
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

    impl ops::Add for Vec4
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

    impl ops::Sub for Vec4
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

    impl ops::Mul for Vec4
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

    impl ops::Div for Vec4
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

    impl ops::Mul<f32> for Vec4
    {
        type Output = Self;

        #[inline]
        fn mul(self, value: f32) -> Self
        {
            unsafe
            {
                Self(_mm_mul_ps(self.0, _mm_set1_ps(value)))
            }
        }
    }

    impl ops::Div<f32> for Vec4
    {
        type Output = Self;

        #[inline]
        fn div(self, value: f32) -> Self
        {
            unsafe
            {
                Self(_mm_div_ps(self.0, _mm_set1_ps(value)))
            }
        }
    }

    impl ops::AddAssign for Vec4
    {
        #[inline]
        fn add_assign(&mut self, other: Self)
        {
            unsafe
            {
                self.0 = _mm_add_ps(self.0, other.0);
            }
        }
    }

    impl fmt::Debug for Vec4
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "({}, {}, {}, {})", self.get_x(), self.get_y(), self.get_z(), self.get_w())
        }
    }
}

pub mod scalar
{
    use std::cmp;
    use std::ops;

    #[derive(Debug, Default, Copy, Clone)]
    pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

    impl Vec4
    {
        #[inline]
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self
        {
            Self(x, y, z, w)
        }

        #[inline]
        pub fn zero() -> Self
        {
            Self::new(0.0, 0.0, 0.0, 0.0)
        }

        #[inline]
        pub fn one() -> Self
        {
            Self::new(1.0, 1.0, 1.0, 1.0)
        }

        #[inline]
        pub fn black() -> Self
        {
            Self::new(0.0, 0.0, 0.0, 1.0)
        }

        #[inline]
        pub fn white() -> Self
        {
            Self::new(1.0, 1.0, 1.0, 1.0)
        }

        #[inline]
        pub fn red() -> Self
        {
            Self::new(1.0, 0.0, 0.0, 1.0)
        }

        #[inline]
        pub fn green() -> Self
        {
            Self::new(0.0, 1.0, 0.0, 1.0)
        }

        #[inline]
        pub fn blue() -> Self
        {
            Self::new(0.0, 0.0, 1.0, 1.0)
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
        pub fn set_w(&mut self, w: f32)
        {
            self.3 = w;
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
        pub fn get_w(self) -> f32
        {
            self.3
        }

        #[inline]
        pub fn clamped(self) -> Self
        {
            Self
            (
                self.0.max(0.0).min(1.0),
                self.1.max(0.0).min(1.0),
                self.2.max(0.0).min(1.0),
                self.3.max(0.0).min(1.0)
            )
        }
    }

    impl cmp::PartialEq for Vec4
    {
        #[inline]
        fn eq(&self, other: &Self) -> bool
        {
            self.0 == other.0 &&
            self.1 == other.1 &&
            self.2 == other.2 &&
            self.3 == other.3
        }
    }

    impl ops::Add for Vec4
    {
        type Output = Self;

        #[inline]
        fn add(self, other: Self) -> Self
        {
            Self
            (
                self.0 + other.0,
                self.1 + other.1,
                self.2 + other.2,
                self.3 + other.3
            )
        }
    }

    impl ops::Sub for Vec4
    {
        type Output = Self;

        #[inline]
        fn sub(self, other: Self) -> Self
        {
            Self
            (
                self.0 - other.0,
                self.1 - other.1,
                self.2 - other.2,
                self.3 - other.3
            )
        }
    }

    impl ops::Mul for Vec4
    {
        type Output = Self;

        #[inline]
        fn mul(self, other: Self) -> Self
        {
            Self
            (
                self.0 * other.0,
                self.1 * other.1,
                self.2 * other.2,
                self.3 * other.3
            )
            
        }
    }

    impl ops::Div for Vec4
    {
        type Output = Self;

        #[inline]
        fn div(self, other: Self) -> Self
        {
            Self
            (
                self.0 / other.0,
                self.1 / other.1,
                self.2 / other.2,
                self.3 / other.3
            )
        }
    }

    impl ops::Mul<f32> for Vec4
    {
        type Output = Self;

        #[inline]
        fn mul(self, value: f32) -> Self
        {
            Self
            (
                self.0 * value,
                self.1 * value,
                self.2 * value,
                self.3 * value
            )
        }
    }

    impl ops::Div<f32> for Vec4
    {
        type Output = Self;

        #[inline]
        fn div(self, value: f32) -> Self
        {
            Self
            (
                self.0 / value,
                self.1 / value,
                self.2 / value,
                self.3 / value
            )
        }
    }

    impl ops::AddAssign for Vec4
    {
        #[inline]
        fn add_assign(&mut self, other: Self)
        {
            self.0 += other.0;
            self.1 += other.1;
            self.2 += other.2;
            self.3 += other.3;
        }
    }
}

macro_rules! vec4_shared
(
    ($module:ident) =>
    {
        impl $module::Vec4
        {
            #[inline]
            pub fn set_r(&mut self, r: f32)
            {
                self.set_x(r);
            }

            #[inline]
            pub fn set_g(&mut self, g: f32)
            {
                self.set_y(g);
            }

            #[inline]
            pub fn set_b(&mut self, b: f32)
            {
                self.set_z(b)
            }

            #[inline]
            pub fn set_a(&mut self, a: f32)
            {
                self.set_w(a)
            }

            #[inline]
            pub fn get_r(self) -> f32
            {
                self.get_x()
            }

            #[inline]
            pub fn get_g(self) -> f32
            {
                self.get_y()
            }

            #[inline]
            pub fn get_b(self) -> f32
            {
                self.get_z()
            }
            
            #[inline]
            pub fn get_a(self) -> f32
            {
                self.get_w()
            }

            #[inline]
            pub fn is_valid(&self) -> bool
            {
                let x = self.get_x();
                if x < 0.0 || 1.0 < x
                {
                    return false;
                }

                let y = self.get_y();
                if y < 0.0 || 1.0 < y
                {
                    return false;
                }

                let z = self.get_z();
                if z < 0.0 || 1.0 < z
                {
                    return false;
                }

                let w = self.get_w();
                if w < 0.0 || 1.0 < w
                {
                    return false;
                }

                true
            }

            #[inline]
            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            pub fn as_quantized_u8_array(&self) -> [u8; 4]
            {
                debug_assert!(self.is_valid());

                [
                    (self.get_r() * 255.99).floor() as u8,
                    (self.get_g() * 255.99).floor() as u8,
                    (self.get_b() * 255.99).floor() as u8,
                    (self.get_a() * 255.99).floor() as u8
                ]
            }
        }
    }
);

#[cfg(target_feature = "sse2")]
vec4_shared!(sse2);
vec4_shared!(scalar);

#[cfg(test)]
mod tests
{
    pub use super::*;

    macro_rules! vec4_tests
    (
        ($module:ident) =>
        {
            mod $module
            {
                use crate::math::types::vec4::$module::Vec4;

                #[test]
                fn new()
                {
                    assert_eq!(Vec4::default(), Vec4::new(0.0, 0.0, 0.0, 0.0));
                    assert_eq!(Vec4::zero(), Vec4::new(0.0, 0.0, 0.0, 0.0));
                    assert_eq!(Vec4::one(), Vec4::new(1.0, 1.0, 1.0, 1.0));
                    assert_eq!(Vec4::black(), Vec4::new(0.0, 0.0, 0.0, 1.0));
                    assert_eq!(Vec4::white(), Vec4::new(1.0, 1.0, 1.0, 1.0));
                    assert_eq!(Vec4::red(), Vec4::new(1.0, 0.0, 0.0, 1.0));
                    assert_eq!(Vec4::green(), Vec4::new(0.0, 1.0, 0.0, 1.0));
                    assert_eq!(Vec4::blue(), Vec4::new(0.0, 0.0, 1.0, 1.0));
                }

                #[test]
                fn calculate()
                {
                    let color = Vec4::new(0.1, 0.2, 0.3, 0.4);
                    assert_eq!(color, color);
 
                    assert_eq!(color + color, Vec4::new(0.2, 0.4, 0.6, 0.8));
                    assert_eq!(color - color, Vec4::new(0.0, 0.0, 0.0, 0.0));
                    assert_eq!(color * color, Vec4::new(0.010000001, 0.040000003, 0.09, 0.16000001));
                    assert_eq!(color / color, Vec4::new(1.0, 1.0, 1.0, 1.0));
                    
                    assert_eq!(color * 2.0, Vec4::new(0.2, 0.4, 0.6, 0.8));
                    assert_eq!(color / 2.0, Vec4::new(0.05, 0.1, 0.15, 0.2));

                    let mut accumulated = Vec4::new(0.0, 0.0, 0.0, 0.0);
                    accumulated += Vec4::black();
                    accumulated += Vec4::white();

                    assert_eq!(accumulated, Vec4::new(1.0, 1.0, 1.0, 2.0));
                }

                #[test]
                fn clamp()
                {
                    let color = Vec4::new(-1.0, 0.0, 1.0, 2.0);
                    assert_eq!(color.clamped(), Vec4::new(0.0, 0.0, 1.0, 1.0));
                }

                #[test]
                fn validate()
                {
                    assert!(Vec4::new(0.0, 0.25, 0.5, 1.0).is_valid() == true);
                    assert!(Vec4::new(-1.0, 0.0, 1.0, 2.0).is_valid() == false);
                }

                #[test]
                fn quantize()
                {
                    let color = Vec4::new(0.0, 0.25, 0.5, 1.0);
                    let quantized = color.as_quantized_u8_array();

                    assert_eq!(quantized[0], 0);
                    assert_eq!(quantized[1], 63);
                    assert_eq!(quantized[2], 127);
                    assert_eq!(quantized[3], 255);
                }
            }
        }
    );

    vec4_tests!(sse2);
    vec4_tests!(scalar);
}
