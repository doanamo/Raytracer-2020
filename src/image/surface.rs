use crate::math::Color;

pub struct Surface
{
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Surface
{
    pub fn new(width: usize, height: usize) -> Self
    {
        debug_assert!(width > 0);
        debug_assert!(height > 0);

        let mut pixels: Vec<Color> = Vec::new();
        pixels.resize(width * height, Color::new(0.0, 0.0, 0.0, 1.0));

        Surface::from(width, height, pixels)
    }

    pub fn from(width: usize, height: usize, pixels: Vec<Color>) -> Self
    {
        debug_assert_eq!(pixels.len(), width * height);

        Surface
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

    pub fn get_pixel_count(&self) -> usize
    {
        self.width * self.height
    }

    pub fn get_data_size(&self) -> usize
    {
        self.get_pixel_count() * std::mem::size_of::<Color>()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color)
    {
        let index = y * self.width + x;
        self.pixels[index] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color
    {
        let index = y * self.width + x;
        self.pixels[index]
    }

    pub fn as_pixel_slice(&self) -> &[Color]
    {
        self.pixels.as_slice()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn new()
    {
        let surface = Surface::new(1920, 1080);
    
        assert_eq!(surface.width, 1920);
        assert_eq!(surface.height, 1080);
        assert_eq!(surface.pixels.len(), 1920 * 1080);

        assert_eq!(surface.get_width(), 1920);
        assert_eq!(surface.get_height(), 1080);
        assert_eq!(surface.get_pixel_count(), 1920 * 1080);
        assert_eq!(surface.get_data_size(), 1920 * 1080 * 4 * 4);
        assert_eq!(surface.get_pixel(0, 0), Color::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    #[should_panic]
    fn new_bad_size()
    {
        Surface::new(0, 0);
    }

    #[test]
    fn from()
    {
        let mut pixels = Vec::with_capacity(1920 * 1080);
        pixels.resize(1920 * 1080, Color::new(0.0, 0.0, 0.0, 1.0));
        pixels[1079 * 1920 + 1919] = Color::new(0.1, 0.2, 0.3, 0.4);

        let surface = Surface::from(1920, 1080, pixels);

        assert_eq!(surface.width, 1920);
        assert_eq!(surface.height, 1080);
        assert_eq!(surface.pixels.len(), 1920 * 1080);

        assert_eq!(surface.get_width(), 1920);
        assert_eq!(surface.get_height(), 1080);
        assert_eq!(surface.get_pixel_count(), 1920 * 1080);
        assert_eq!(surface.get_data_size(), 1920 * 1080 * 4 * 4);
        assert_eq!(surface.get_pixel(0, 0), Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(surface.get_pixel(1919, 1079), Color::new(0.1, 0.2, 0.3, 0.4));
    }

    #[test]
    #[should_panic]
    fn from_bad_size()
    {
        let mut pixels = Vec::with_capacity(1920 * 1080);
        pixels.resize(1920 * 1080, Color::new(0.0, 0.0, 0.0, 1.0));

        Surface::from(1024, 576, pixels);
    }

    #[test]
    fn set_pixel()
    {
        let mut surface = Surface::new(1920, 1080);

        surface.set_pixel(1919, 1079, Color::new(0.1, 0.2, 0.3, 0.4));
        assert_eq!(surface.get_pixel(1919, 1079), Color::new(0.1, 0.2, 0.3, 0.4));
    }

    #[test]
    fn as_pixel_slice()
    {
        let mut surface = Surface::new(1920, 1080);
        surface.set_pixel(1919, 1079, Color::new(0.1, 0.2, 0.3, 0.4));

        let surface_slice = surface.as_pixel_slice();
        assert_eq!(surface_slice.len(), 1920 * 1080);
        assert_eq!(surface_slice[1079 * 1920 + 1919], Color::new(0.1, 0.2, 0.3, 0.4));
    }
}
