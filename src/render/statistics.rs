use std::iter;

pub struct Statistics
{
    pub pixels: usize,
    pub subpixels: usize,
    pub samples: usize,
    pub intersections: usize,
    pub scatters: usize,
    pub max_scatters: u16
}

impl Default for Statistics
{
    fn default() -> Self
    {
        Self
        {
            pixels: 0,
            subpixels: 0,
            samples: 0,
            intersections: 0,
            scatters: 0,
            max_scatters: 0
        }
    }
}

impl Statistics
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn new_pixel() -> Self
    {
        Self
        {
            pixels: 1,
            subpixels: 0,
            samples: 0,
            intersections: 0,
            scatters: 0,
            max_scatters: 0
        }
    }

    pub fn accumulated(&self, other: &Self) -> Self
    {
        Self
        {
            pixels: self.pixels + other.pixels,
            subpixels: self.subpixels + other.subpixels,
            samples: self.samples + other.samples,
            intersections: self.intersections + other.intersections,
            scatters: self.scatters + other.scatters,
            max_scatters: self.max_scatters.max(other.max_scatters)
        }
    }

    pub fn print(&self)
    {
        println!("Printing render statistics:");
        println!("  Pixels:        {}", self.pixels);
        println!("  Subpixels:     {} ({} per pixel)", self.subpixels, self.subpixels / self.pixels);
        println!("  Samples:       {} ({:.2} per pixel)", self.samples, self.samples as f32 / self.pixels as f32);
        println!("  Intersections: {} ({:.2} per pixel)", self.intersections, self.intersections as f32 / self.pixels as f32);
        println!("  Scatters:      {} ({} max)", self.scatters, self.max_scatters);
    }
}

impl iter::Sum<Self> for Statistics
{
    fn sum<I: Iterator<Item = Self>>(iterator: I) -> Self
    {
        iterator.fold(Self::new(), |a, b|
        {
            a.accumulated(&b)
        })
    }
}
