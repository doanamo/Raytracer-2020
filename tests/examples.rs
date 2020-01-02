mod examples
{
    use raytracer::math::{ Vec3, Color };
    use raytracer::image;
    use raytracer::render;
    use render::primitive;
    use render::material;

    #[test]
    fn spheres()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 8,
            scatter_limit: 32,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -0.6, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 1.0, -0.2)))
            .set_field_of_view(55.0);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_primitive(primitive::Sphere::new(Vec3::new(0.3, 0.5, -0.3), 0.2, material::Refractive::from(Color::new(1.0, 1.0, 1.0, 1.0), 1.008)))
            .add_primitive(primitive::Sphere::new(Vec3::new(-0.3, 0.5, -0.3), -0.2, material::Refractive::from(Color::new(1.0, 1.0, 1.0, 1.0), 1.3)))
            .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 1.4, 0.0), 0.5, material::Diffuse::from(Color::new(0.8, 0.3, 0.3, 1.0))))
            .add_primitive(primitive::Sphere::new(Vec3::new(0.8, 1.0, -0.1), 0.4, material::Metalic::from(Color::new(0.8, 0.8, 0.8, 1.0), 0.0)))
            .add_primitive(primitive::Sphere::new(Vec3::new(-0.8, 1.0, -0.1), 0.4, material::Metalic::from(Color::new(0.8, 0.8, 0.8, 1.0), 0.8)))
            .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, material::Diffuse::from(Color::new(0.8, 0.8, 0.0, 1.0))));

        let setup = render::Setup
        {
            parameters,
            scene
        };

        setup.save("examples/spheres.json").expect("Saving setup file failed!");

        let _ = std::fs::remove_dir_all("target/tests/examples/spheres/");

        let test_parameters = render::Parameters
        {
            image_width: setup.parameters.image_width / 16,
            image_height: setup.parameters.image_height / 16,
            antialias_samples: 1,
            scatter_limit: 8,
            ..setup.parameters
        };

        let image = render::Renderer::new()
            .set_parameters(&test_parameters)
            .set_scene(&setup.scene)
            .render();

        image::Writer::new(image::FormatPNG::new())
            .input(&image).output("target/tests/examples/spheres/output.png")
            .save().expect("Failed to save rendered image!");
    }

    #[test]
    fn diffuse()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 4,
            scatter_limit: 32,
            debug_mode: Some(render::DebugMode::Diffuse),
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -0.6, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 1.0, -0.2)))
            .set_field_of_view(55.0);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 0.5, -0.1), 0.4, material::Diffuse::from(Color::new(0.8, 0.8, 0.8, 1.0))))
            .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, material::Diffuse::from(Color::new(0.8, 0.8, 0.8, 1.0))));

        let setup = render::Setup
        {
            parameters,
            scene
        };

        setup.save("examples/diffuse.json").expect("Saving setup file failed!");

        let _ = std::fs::remove_dir_all("target/tests/examples/diffuse/");

        let test_parameters = render::Parameters
        {
            image_width: setup.parameters.image_width / 16,
            image_height: setup.parameters.image_height / 16,
            antialias_samples: 1,
            scatter_limit: 8,
            ..setup.parameters
        };

        let image = render::Renderer::new()
            .set_parameters(&test_parameters)
            .set_scene(&setup.scene)
            .render();

        image::Writer::new(image::FormatPNG::new())
            .input(&image).output("target/tests/examples/diffuse/output.png")
            .save().expect("Failed to save rendered image!");
    }

    #[test]
    fn normals()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 4,
            scatter_limit: 1,
            debug_mode: Some(render::DebugMode::Normals),
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -0.6, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 1.0, -0.2)))
            .set_field_of_view(55.0);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 0.5, -0.1), 0.4, material::Diffuse::from(Color::new(0.8, 0.8, 0.8, 1.0))))
            .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, material::Diffuse::from(Color::new(0.8, 0.8, 0.8, 1.0))));

        let setup = render::Setup
        {
            parameters,
            scene
        };

        setup.save("examples/normals.json").expect("Saving setup file failed!");

        let _ = std::fs::remove_dir_all("target/tests/examples/normals/");

        let test_parameters = render::Parameters
        {
            image_width: setup.parameters.image_width / 16,
            image_height: setup.parameters.image_height / 16,
            antialias_samples: 1,
            ..setup.parameters
        };

        let image = render::Renderer::new()
            .set_parameters(&test_parameters)
            .set_scene(&setup.scene)
            .render();

        image::Writer::new(image::FormatPNG::new())
            .input(&image).output("target/tests/examples/normals/output.png")
            .save().expect("Failed to save rendered image!");
    }
}
