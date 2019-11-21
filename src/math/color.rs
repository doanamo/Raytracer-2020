use std::ops;

#[derive(Copy, Clone)]
pub struct Color
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color
{
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color
    {
        Color
        {
            r: red,
            g: green,
            b: blue,
            a: alpha
        }
    }

    pub fn as_u8_array(&self) -> [u8; 4]
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

    pub fn add_rgba(self, color: Color) -> Color
    {
        Color
        {
            r: self.r + color.r,
            g: self.g + color.g,
            b: self.b + color.b,
            a: self.a + color.a
        }
    }
    
    pub fn mul_rgb(self, factor: f32) -> Color
    {
        Color
        {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
            a: self.a
        }
    }

    pub fn mul_rgba(self, factor: f32) -> Color
    {
        Color
        {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
            a: self.a * factor
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

impl ops::AddAssign for Color
{
    fn add_assign(&mut self, other: Self)
    {
        *self = Self
        {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a
        };
    }
}

impl ops::MulAssign for Color
{
    fn mul_assign(&mut self, other: Self)
    {
        *self = Self
        {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a
        };
    }
}

impl ops::DivAssign<u32> for Color
{
    fn div_assign(&mut self, other: u32)
    {
        *self = Self
        {
            r: self.r / other as f32,
            g: self.g / other as f32,
            b: self.b / other as f32,
            a: self.a / other as f32
        };
    }
}

impl ops::DivAssign<u16> for Color
{
    fn div_assign(&mut self, other: u16)
    {
        *self = Self
        {
            r: self.r / other as f32,
            g: self.g / other as f32,
            b: self.b / other as f32,
            a: self.a / other as f32
        };
    }
}
