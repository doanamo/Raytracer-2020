use std::cmp;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Default for Color
{
    fn default() -> Self
    {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl Color
{
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self
    {
        Color
        {
            r: red,
            g: green,
            b: blue,
            a: alpha
        }
    }

    pub fn zero() -> Self
    {
        Color
        {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        }
    }

    pub fn white() -> Self
    {
        Color
        {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0
        }
    }

    pub fn black() -> Self
    {
        Color
        {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    }

    pub fn red() -> Self
    {
        Color
        {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    }

    pub fn green() -> Self
    {
        Color
        {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0
        }
    }

    pub fn blue() -> Self
    {
        Color
        {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0
        }
    }

    pub fn as_quantized_u8_array(&self) -> [u8; 4]
    {
        [
            (self.r * 255.99).floor() as u8,
            (self.g * 255.99).floor() as u8,
            (self.b * 255.99).floor() as u8,
            (self.a * 255.99).floor() as u8
        ]
    }

    pub fn add_rgb(self, color: Color) -> Color
    {
        Color
        {
            r: self.r + color.r,
            g: self.g + color.g,
            b: self.b + color.b,
            a: self.a
        }
    }

    pub fn sub_rgb(self, color: Color) -> Color
    {
        Color
        {
            r: self.r - color.r,
            g: self.g - color.g,
            b: self.b - color.b,
            a: self.a
        }
    }
    
    pub fn mul_rgb(self, value: f32) -> Color
    {
        Color
        {
            r: self.r * value,
            g: self.g * value,
            b: self.b * value,
            a: self.a
        }
    }

    pub fn div_rgb(self, value: f32) -> Color
    {
        Color
        {
            r: self.r / value,
            g: self.g / value,
            b: self.b / value,
            a: self.a
        }
    }

    pub fn clamped(self) -> Color
    {
        Color
        {
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0),
            a: self.a.max(0.0).min(1.0)
        }
    }

    pub fn is_valid(&self) -> bool
    {
        if self.r < 0.0 || 1.0 < self.r
        {
            return false;
        }

        if self.g < 0.0 || 1.0 < self.g
        {
            return false;
        }

        if self.b < 0.0 || 1.0 < self.b
        {
            return false;
        }

        if self.a < 0.0 || 1.0 < self.a
        {
            return false;
        }

        return true;
    }
}

impl cmp::PartialEq for Color
{
    fn eq(&self, other: &Self) -> bool
    {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b &&
        self.a == other.a
    }
}

impl ops::Add<Color> for Color
{
    type Output = Color;

    fn add(self, other: Self) -> Color
    {
        Color
        {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a
        }
    }
}

impl ops::Sub<Color> for Color
{
    type Output = Color;

    fn sub(self, other: Self) -> Color
    {
        Color
        {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a
        }
    }
}

impl ops::Mul<Color> for Color
{
    type Output = Color;

    fn mul(self, other: Self) -> Color
    {
        Color
        {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a
        }
    }
}

impl ops::Div<Color> for Color
{
    type Output = Color;

    fn div(self, other: Self) -> Color
    {
        Color
        {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
            a: self.a / other.a
        }
    }
}

impl ops::Mul<f32> for Color
{
    type Output = Color;

    fn mul(self, value: f32) -> Color
    {
        Color
        {
            r: self.r * value,
            g: self.g * value,
            b: self.b * value,
            a: self.a * value
        }
    }
}

impl ops::Div<f32> for Color
{
    type Output = Color;

    fn div(self, value: f32) -> Color
    {
        Color
        {
            r: self.r / value,
            g: self.g / value,
            b: self.b / value,
            a: self.a / value
        }
    }
}

impl ops::AddAssign for Color
{
    fn add_assign(&mut self, other: Self)
    {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
        self.a += other.a;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn new()
    {
        assert_eq!(Color::default(), Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::zero(), Color::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Color::white(), Color::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Color::black(), Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::red(), Color::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::green(), Color::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(Color::blue(), Color::new(0.0, 0.0, 1.0, 1.0));
    }

    #[test]
    fn quantize()
    {
        let color = Color::new(0.0, 0.25, 0.5, 1.0);
        let quantized = color.as_quantized_u8_array();

        assert_eq!(quantized[0], 0);
        assert_eq!(quantized[1], 63);
        assert_eq!(quantized[2], 127);
        assert_eq!(quantized[3], 255);
    }

    #[test]
    fn calculate()
    {
        let color = Color::new(0.1, 0.2, 0.3, 0.4);
        assert_eq!(color, color);

        assert_eq!(color.add_rgb(color), Color::new(0.2, 0.4, 0.6, 0.4));
        assert_eq!(color.sub_rgb(color), Color::new(0.0, 0.0, 0.0, 0.4));
        assert_eq!(color.mul_rgb(2.0), Color::new(0.2, 0.4, 0.6, 0.4));
        assert_eq!(color.div_rgb(2.0), Color::new(0.05, 0.1, 0.15, 0.4));
        
        assert_eq!(color + color, Color::new(0.2, 0.4, 0.6, 0.8));
        assert_eq!(color - color, Color::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(color * color, Color::new(0.010000001, 0.040000003, 0.09, 0.16000001));
        assert_eq!(color / color, Color::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(color * 2.0, Color::new(0.2, 0.4, 0.6, 0.8));
        assert_eq!(color / 2.0, Color::new(0.05, 0.1, 0.15, 0.2));

        let mut accumulated = Color::new(0.0, 0.0, 0.0, 0.0);
        accumulated += Color::black();
        accumulated += Color::white();

        assert_eq!(accumulated, Color::new(1.0, 1.0, 1.0, 2.0));
    }

    #[test]
    fn clamp()
    {
        let color = Color::new(-1.0, 0.0, 1.0, 2.0);
        assert_eq!(color.clamped(), Color::new(0.0, 0.0, 1.0, 1.0));
    }

    #[test]
    fn validate()
    {
        assert!(Color::new(0.0, 0.25, 0.5, 1.0).is_valid() == true);
        assert!(Color::new(-1.0, 0.0, 1.0, 2.0).is_valid() == false);
    }
}
