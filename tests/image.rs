mod image
{
    use std::path::Path;
    use raytracer::image;
    use raytracer::math;

    #[test]
    fn write_pnm()
    {
        let _ = std::fs::remove_dir_all("target/tests/image/write_pnm/");

        let mut pixels: Vec<math::Color> = Vec::with_capacity(16 * 16);

        for _x in 0..16
        {
            for _y in 0..16
            {
                pixels.push(math::Color::red());
            }
        }

        let image = image::Image::from(16, 16, pixels);

        image::Writer::new(image::FormatPNM::new())
            .input(&image).output(Path::new("target/tests/image/write_pnm/output.pnm"))
            .save().expect("Failed to save image!");

        assert!(file_diff::diff("target/tests/image/write_pnm/output.pnm", "tests/references/image_write.pnm"));
    }

    #[test]
    fn write_png()
    {
        let _ = std::fs::remove_dir_all("target/tests/image/write_png/");

        let mut pixels: Vec<math::Color> = Vec::with_capacity(16 * 16);

        for _x in 0..16
        {
            for _y in 0..16
            {
                pixels.push(math::Color::green());
            }
        }

        let image = image::Image::from(16, 16, pixels);

        image::Writer::new(image::FormatPNG::new())
            .input(&image).output(Path::new("target/tests/image/write_png/output.png"))
            .save().expect("Failed to save image!");

        assert!(file_diff::diff("target/tests/image/write_png/output.png", "tests/references/image_write.png"));
    }
}
