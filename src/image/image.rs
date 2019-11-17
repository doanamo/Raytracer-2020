use crate::math::Color;

pub struct Image
{
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Image
{
    pub fn new(width: usize, height: usize) -> Image
    {
        let mut data: Vec<Color> = Vec::new();
        data.resize(width * height, Color::new(0.0, 0.0, 0.0, 1.0));

        Image
        {
            width,
            height,
            data,
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
        self.data[index] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color
    {
        let index = y * self.width + x;
        self.data[index].clone()
    }

    pub fn _get_data(&self) -> &Vec<Color>
    {
        &self.data
    }
}
