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
}
