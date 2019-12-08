use crate::math::Color;

pub struct Image
{
    width: usize,
    height: usize,
    pub pixels: Vec<Color>,
}

impl Image
{
    pub fn new(width: usize, height: usize) -> Image
    {
        let mut pixels: Vec<Color> = Vec::new();
        pixels.resize(width * height, Color::new(0.0, 0.0, 0.0, 1.0));

        Image
        {
            width,
            height,
            pixels
        }
    }

    pub fn get_width(&self) -> usize
    {
        self.width
    }

    pub fn get_height(&self) -> usize
    {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color)
    {
        let index = y * self.width + x;
        self.pixels[index] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color
    {
        let index = y * self.width + x;
        self.pixels[index].clone()
    }
}
