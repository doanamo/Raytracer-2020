mod math
{
    use std::path::Path;
    use std::io::BufWriter;
    use std::fs::OpenOptions;
    use serde::Serialize;
    use raytracer::math::Vec2;
    use raytracer::math::Vec3;
    use raytracer::math::Vec4;
    use raytracer::math::Ray;
    use raytracer::math::Sphere;
    use raytracer::math::Intersectable;

    #[test]
    fn intersect_ray_sphere()
    {
        let sphere = Sphere::new(Vec3::new(0.0, 10.0, 0.0), 1.0);
        
        let ray_forward = Ray::new(Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 0.0);
        let intersection = sphere.intersect(&ray_forward, 0.0001, std::f32::MAX).unwrap();
        
        assert_eq!(intersection.point, Vec3::new(0.0, 9.0, 0.0));
        assert_eq!(intersection.normal, Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(intersection.length, 7.0);

        let ray_backward = Ray::new(Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, -1.0, 0.0), 0.0);
        assert!(sphere.intersect(&ray_backward, 0.0001, std::f32::MAX).is_none());
    }

    #[test]
    fn serialize_vec2()
    {
        let directory = Path::new("target/tests/math/serialize_vec2/");

        {
            #[derive(Serialize)]
            struct Output
            {
                vector: Vec2
            }

            let _ = std::fs::remove_dir_all(directory);
            let _ = std::fs::create_dir_all(directory);

            let file = OpenOptions::new().write(true).truncate(true).create(true).open(directory.join("output.json")).unwrap();
            let writer = BufWriter::new(file);

            let output = Output
            {
                vector: Vec2::new(1.0, 2.0)
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
                vector: Vec3
            }

            let _ = std::fs::remove_dir_all(directory);
            let _ = std::fs::create_dir_all(directory);

            let file = OpenOptions::new().write(true).truncate(true).create(true).open(directory.join("output.json")).unwrap();
            let writer = BufWriter::new(file);

            let output = Output
            {
                vector: Vec3::new(1.0, 2.0, 3.0)
            };

            serde_json::to_writer_pretty(writer, &output).unwrap();
        }
        
        assert!(file_diff::diff(directory.join("output.json").to_str().unwrap(), "tests/references/serialize_vec3.json"));
    }

    #[test]
    fn serialize_vec4()
    {
        let directory = Path::new("target/tests/math/serialize_vec4/");

        {
            #[derive(Serialize)]
            struct Output
            {
                vector: Vec4
            }

            let _ = std::fs::remove_dir_all(directory);
            let _ = std::fs::create_dir_all(directory);

            let file = OpenOptions::new().write(true).truncate(true).create(true).open(directory.join("output.json")).unwrap();
            let writer = BufWriter::new(file);

            let output = Output
            {
                vector: Vec4::new(0.25, 0.5, 0.75, 1.0)
            };

            serde_json::to_writer_pretty(writer, &output).unwrap();
        }
        
        assert!(file_diff::diff(directory.join("output.json").to_str().unwrap(), "tests/references/serialize_vec4.json"));
    }
}
