use crate::math::Color;

pub struct Image
{
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Image
{
    pub fn new(width: usize, height: usize) -> Self
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

    pub fn from(width: usize, height: usize, pixels: Vec<Color>) -> Self
    {
        debug_assert_eq!(pixels.len(), width * height);

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

    pub fn get_pixel(&self, x: usize, y: usize) -> &Color
    {
        let index = y * self.width + x;
        &self.pixels[index]
    }

    pub fn get_pixel_data(&self) -> &[Color]
    {
        self.pixels.as_slice()
    }

    pub fn get_pixel_count(&self) -> usize
    {
        self.width * self.height
    }

    pub fn get_data_size(&self) -> usize
    {
        self.get_pixel_count() * std::mem::size_of::<Color>()
    }
}
