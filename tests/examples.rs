mod examples
{
    use raytracer::math::Vec3;
    use raytracer::math::Vec4;
    use raytracer::image;
    use raytracer::render;
    use render::objects;
    use render::materials;

    #[test]
    fn spheres()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 16,
            scatter_limit: 16,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -0.6, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 1.0, -0.2)))
            .set_field_of_view(55.0);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_object(objects::Sphere::new(Vec3::new(0.3, 0.5, -0.3), 0.2, materials::Refractive::new(Vec4::new(1.0, 1.0, 1.0, 1.0), 1.008)))
            .add_object(objects::Sphere::new(Vec3::new(-0.3, 0.5, -0.3), -0.2, materials::Refractive::new(Vec4::new(1.0, 1.0, 1.0, 1.0), 1.3)))
            .add_object(objects::Sphere::new(Vec3::new(0.0, 1.4, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.8, 0.3, 0.3, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(0.8, 1.0, -0.1), 0.4, materials::Metallic::new(Vec4::new(0.8, 0.8, 0.8, 1.0), 0.0)))
            .add_object(objects::Sphere::new(Vec3::new(-0.8, 1.0, -0.1), 0.4, materials::Metallic::new(Vec4::new(0.8, 0.8, 0.8, 1.0), 0.8)))
            .add_object(objects::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.0, 1.0))));

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
    fn metallic()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 200,
            antialias_samples: 16,
            scatter_limit: 16,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -5.5, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 0.0, 0.0)))
            .set_field_of_view(20.0);
        
        let mut scene = render::Scene::new()
            .set_camera(camera)
            .add_object(objects::Sphere::new(Vec3::new(0.0, 1.0, -600.5), 600.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.0, 1.0))));

        for x in 0..=8
        {
            scene = scene.add_object(
                objects::Sphere::new(Vec3::new(1.0 * (x as f32) - 4.0, 0.0, -0.002 * ((x - 4) as f32).abs()), 0.5,
                materials::Metallic::new(Vec4::new(0.9, 0.9, 0.9, 1.0), 1.0 / 8.0 * (x as f32)))
            );
        }

        let setup = render::Setup
        {
            parameters,
            scene
        };

        setup.save("examples/metallic.json").expect("Saving setup file failed!");

        let _ = std::fs::remove_dir_all("target/tests/examples/metallic/");

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
            .input(&image).output("target/tests/examples/metallic/output.png")
            .save().expect("Failed to save rendered image!");
    }

    #[test]
    fn focus()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 16,
            scatter_limit: 16,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.8, 1.2, 1.0))
            .set_look_at(Some(Vec3::new(0.0, 0.0, 0.0)))
            .set_field_of_view(55.0)
            .set_aperture_size(0.1)
            .set_focus_on_look_at(-0.25);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_object(objects::Sphere::new(Vec3::new(0.0, 0.0, -100.5), 100.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.0, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(1.3, 0.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.3, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(-1.3, 0.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.3, 0.6, 0.3, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(0.0, 1.3, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.6, 0.2, 0.2, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(0.0, -1.3, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.3, 0.3, 0.6, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(1.0, 1.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(1.0, 0.3, 0.3, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(-1.0, -1.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.3, 1.0, 0.3, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(-1.0, 1.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(1.0, 0.6, 0.3, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(1.0, -1.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.3, 0.3, 1.0, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, materials::Metallic::new(Vec4::new(0.8, 0.8, 0.8, 1.0), 0.0)));

        let setup = render::Setup
        {
            parameters,
            scene
        };

        setup.save("examples/focus.json").expect("Saving setup file failed!");

        let _ = std::fs::remove_dir_all("target/tests/examples/focus/");

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
            .input(&image).output("target/tests/examples/focus/output.png")
            .save().expect("Failed to save rendered image!");
    }

    #[test]
    fn diffuse()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 16,
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
            .add_object(objects::Sphere::new(Vec3::new(0.0, 0.5, -0.1), 0.4, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.8, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.8, 1.0))));

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
            .add_object(objects::Sphere::new(Vec3::new(0.0, 0.5, -0.1), 0.4, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.8, 1.0))))
            .add_object(objects::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.8, 1.0))));

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
