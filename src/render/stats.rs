use std::iter;

pub struct RenderStats
{
    pub pixels: usize,
    pub subpixels: usize,
    pub samples: usize,
    pub intersections: usize,
    pub scatters: usize,
    pub max_scatters: u16
}

impl RenderStats
{
    pub fn new() -> Self
    {
        RenderStats
        {
            pixels: 0,
            subpixels: 0,
            samples: 0,
            intersections: 0,
            scatters: 0,
            max_scatters: 0
        }
    }

    pub fn new_pixel() -> Self
    {
        RenderStats
        {
            pixels: 1,
            subpixels: 0,
            samples: 0,
            intersections: 0,
            scatters: 0,
            max_scatters: 0
        }
    }

    pub fn print(&self)
    {
        println!("Printing render stats:");
        println!("  Pixels:        {}", self.pixels);
        println!("  Subpixels:     {} ({} per pixel)", self.subpixels, self.subpixels / self.pixels);
        println!("  Samples:       {} ({:.2} per pixel)", self.samples, self.samples as f32 / self.pixels as f32);
        println!("  Intersections: {} ({:.2} per pixel)", self.intersections, self.intersections as f32 / self.pixels as f32);
        println!("  Scatters:      {} ({} max)", self.scatters, self.max_scatters);
    }
}

impl iter::Sum<Self> for RenderStats
{
    fn sum<I: Iterator<Item = Self>>(iterator: I) -> Self
    {
        iterator.fold(Self::new(), |a, b|
        {
            Self
            {
                pixels: a.pixels + b.pixels,
                subpixels: a.subpixels + b.subpixels,
                samples: a.samples + b.samples,
                intersections: a.intersections + b.intersections,
                scatters: a.scatters + b.scatters,
                max_scatters: std::cmp::max(a.max_scatters, b.max_scatters)
            }
        })
    }
}
