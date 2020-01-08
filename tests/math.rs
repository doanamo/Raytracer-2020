mod math
{
    use std::path::Path;
    use std::io::BufWriter;
    use std::fs::OpenOptions;
    use serde::Serialize;
    use raytracer::math;
    use raytracer::math::Intersectable;

    #[test]
    fn intersect_ray_sphere()
    {
        let sphere = math::Sphere::new(math::Vec3::new(0.0, 10.0, 0.0), 1.0);
        
        let ray_forward = math::Ray::new(math::Vec3::new(0.0, 2.0, 0.0), math::Vec3::new(0.0, 1.0, 0.0));
        let intersection = sphere.intersect(&ray_forward, 0.0001, std::f32::MAX).unwrap();
        
        assert_eq!(intersection.point, math::Vec3::new(0.0, 9.0, 0.0));
        assert_eq!(intersection.normal, math::Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(intersection.length, 7.0);

        let ray_backward = math::Ray::new(math::Vec3::new(0.0, 2.0, 0.0), math::Vec3::new(0.0, -1.0, 0.0));
        assert!(sphere.intersect(&ray_backward, 0.0001, std::f32::MAX).is_none());
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
