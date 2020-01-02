mod image
{
    use std::path::Path;
    use raytracer::image;
    use raytracer::math;

    #[test]
    fn write_pnm()
    {
        let directory = Path::new("target/tests/image/write_pnm/");

        {
            let _ = std::fs::remove_dir_all(directory);
            let _ = std::fs::create_dir_all(directory);

            let mut pixels: Vec<math::Color> = Vec::with_capacity(16 * 16);

            for _x in 0..16
            {
                for _y in 0..16
                {
                    pixels.push(math::Color::red());
                }
            }
    
            let image = image::Surface::from(16, 16, pixels);
    
            image::Writer::new(image::FormatPNM::new())
                .input(&image).output(directory.join("output.pnm"))
                .save().expect("Failed to save image!");    
        }
        

        assert!(file_diff::diff(directory.join("output.pnm").to_str().unwrap(), "tests/references/write_image.pnm"));
    }

    #[test]
    fn write_png()
    {
        let directory = Path::new("target/tests/image/write_png/");

        {
            let _ = std::fs::remove_dir_all(directory);
            let _ = std::fs::create_dir_all(directory);

            let mut pixels: Vec<math::Color> = Vec::with_capacity(16 * 16);

            for _x in 0..16
            {
                for _y in 0..16
                {
                    pixels.push(math::Color::green());
                }
            }

            let image = image::Surface::from(16, 16, pixels);

            image::Writer::new(image::FormatPNG::new())
                .input(&image).output(directory.join("output.png"))
                .save().expect("Failed to save image!");
        }

        assert!(file_diff::diff(directory.join("output.png").to_str().unwrap(), "tests/references/write_image.png"));
    }
}
