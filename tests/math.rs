mod math
{
    use std::path::Path;
    use std::io::BufWriter;
    use std::fs::OpenOptions;
    use serde::Serialize;
    use raytracer::math;

    #[test]
    fn intersect_sphere()
    {
        // move intersection code from renderer to math
    }

    #[test]
    fn serialize_color()
    {
        let directory = Path::new("target/tests/math/serialize_color/");

        {
            #[derive(Serialize)]
            struct Output
            {
                color: math::Color
            }

            let _ = std::fs::remove_dir_all(directory);
            let _ = std::fs::create_dir_all(directory);

            let file = OpenOptions::new().write(true).truncate(true).create(true).open(directory.join("output.json")).unwrap();
            let writer = BufWriter::new(file);

            let output = Output
            {
                color: math::Color::new(0.25, 0.5, 0.75, 1.0)
            };

            serde_json::to_writer_pretty(writer, &output).unwrap();
        }
        
        assert!(file_diff::diff(directory.join("output.json").to_str().unwrap(), "tests/references/serialize_color.json"));
    }

    #[test]
    fn serialize_vec2()
    {
        let directory = Path::new("target/tests/math/serialize_vec2/");

        {
            #[derive(Serialize)]
            struct Output
            {
                position: math::Vec2
            }

            let _ = std::fs::remove_dir_all(directory);
            let _ = std::fs::create_dir_all(directory);

            let file = OpenOptions::new().write(true).truncate(true).create(true).open(directory.join("output.json")).unwrap();
            let writer = BufWriter::new(file);

            let output = Output
            {
                position: math::Vec2::new(1.0, 2.0)
            };

            serde_json::to_writer_pretty(writer, &output).unwrap();
        }
        
        assert!(file_diff::diff(directory.join("output.json").to_str().unwrap(), "tests/references/serialize_vec2.json"));
    }

    #[test]
    fn serialize_vec3()
    {
        let directory = Path::new("target/tests/math/serialize_vec3/");

        {
            #[derive(Serialize)]
            struct Output
            {
                position: math::Vec3
            }

            let _ = std::fs::remove_dir_all(directory);
            let _ = std::fs::create_dir_all(directory);

            let file = OpenOptions::new().write(true).truncate(true).create(true).open(directory.join("output.json")).unwrap();
            let writer = BufWriter::new(file);

            let output = Output
            {
                position: math::Vec3::new(1.0, 2.0, 3.0)
            };

            serde_json::to_writer_pretty(writer, &output).unwrap();
        }
        
        assert!(file_diff::diff(directory.join("output.json").to_str().unwrap(), "tests/references/serialize_vec3.json"));
    }
}
