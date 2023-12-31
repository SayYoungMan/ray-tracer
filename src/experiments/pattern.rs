use std::{error::Error, f64::consts::PI};

use crate::{
    camera::Camera,
    color::Color,
    lights::PointLight,
    materials::Material,
    patterns::{
        blended::Blended, checker::Checker, gradient::Gradient, radial_gradient::RadialGradient,
        ring::Ring, solid::Solid, stripe::Stripe, Pattern,
    },
    shapes::{plane::Plane, sphere::Sphere, Shape},
    transformation::{rotation_x, rotation_y, rotation_z, scaling, translation, view_transform},
    tuples::{Point, Vector},
    world::World,
};

pub fn draw_chapter_10_first_page() -> Result<(), Box<dyn Error>> {
    let mut floor = Plane::new();
    let mut floor_material = Material::new();
    floor_material.pattern = Box::new(Checker::new(
        Box::new(Solid::new(Color::white())),
        Box::new(Solid::new(Color::black())),
    ));
    floor.set_material(floor_material);

    let mut wall = Plane::new();
    wall.set_transformation(
        translation(0.0, 0.0, 3.0)
            * rotation_z(PI / 4.0)
            * scaling(0.5, 0.5, 0.5)
            * rotation_x(PI / 2.0),
    );
    let mut wall_material = Material::new();
    wall_material.pattern = Box::new(Stripe::new(
        Box::new(Solid::new(Color::white())),
        Box::new(Solid::new(Color::black())),
    ));
    wall.set_material(wall_material);

    let mut big_sphere = Sphere::new();
    let mut big_sphere_material = Material::new();
    big_sphere_material.pattern = Box::new(Ring::new(
        Box::new(Solid::new(Color(0.56, 0.93, 0.56))),
        Box::new(Solid::new(Color(0.0, 0.2, 0.13))),
    ));
    big_sphere_material
        .pattern
        .as_mut()
        .set_transformation(scaling(0.2, 1.0, 1.5));
    big_sphere.set_material(big_sphere_material);
    big_sphere.set_transformation(
        translation(-0.5, 1.0, 0.0) * rotation_z(-PI / 4.0) * rotation_y(PI / 3.0),
    );

    let mut small_sphere = Sphere::new();
    let mut small_sphere_material = Material::new();
    small_sphere_material.pattern =
        Box::new(Gradient::new(Color(1.0, 0.0, 0.0), Color(0.0, 1.0, 0.0)));
    small_sphere_material
        .pattern
        .as_mut()
        .set_transformation(translation(1.0, 0.0, 0.0) * scaling(2.0, 2.0, 2.0));
    small_sphere.set_material(small_sphere_material);
    small_sphere.set_transformation(
        scaling(0.33, 0.33, 0.33)
            * translation(5.0, 1.0, -2.0)
            * rotation_y(-PI / 4.0)
            * rotation_z(PI / 4.0),
    );

    let world = World {
        objects: vec![
            Box::new(floor),
            Box::new(wall),
            Box::new(big_sphere),
            Box::new(small_sphere),
        ],
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color(0.5, 0.5, 0.5)),
    };

    let mut camera = Camera::new(150, 75, PI / 3.0);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);
    canvas.to_ppm("images/chapter_10_first_page.ppm")?;

    Ok(())
}

pub fn radial_gradient_floor() -> Result<(), Box<dyn Error>> {
    let mut floor = Plane::new();
    let mut floor_material = Material::new();
    floor_material.pattern = Box::new(RadialGradient::new(Color::white(), Color::black()));
    floor.set_material(floor_material);

    let world = World {
        objects: vec![Box::new(floor)],
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0)),
    };

    let mut camera = Camera::new(150, 75, PI / 3.0);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);
    canvas.to_ppm("images/radial_gradient_floor.ppm")?;

    Ok(())
}

pub fn nested_pattern_floor() -> Result<(), Box<dyn Error>> {
    let mut floor = Plane::new();

    let mut stripe_a = Stripe::new(
        Box::new(Solid::new(Color(1.0, 0.753, 0.796))),
        Box::new(Solid::new(Color(1.0, 0.0, 1.0))),
    );
    stripe_a.set_transformation(scaling(0.2, 0.2, 0.2) * rotation_y(PI / 4.0));

    let mut stripe_b = Stripe::new(
        Box::new(Solid::new(Color(0.6, 0.6, 0.6))),
        Box::new(Solid::new(Color(0.3, 0.3, 0.3))),
    );
    stripe_b.set_transformation(scaling(0.2, 0.2, 0.2) * rotation_y(-PI / 4.0));

    let mut floor_material = Material::new();
    floor_material.pattern = Box::new(Checker::new(Box::new(stripe_a), Box::new(stripe_b)));
    floor.set_material(floor_material);

    let world = World {
        objects: vec![Box::new(floor)],
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0)),
    };

    let mut camera = Camera::new(150, 75, PI / 3.0);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);
    canvas.to_ppm("images/nested_pattern_floor.ppm")?;

    Ok(())
}

pub fn blended_pattern_floor() -> Result<(), Box<dyn Error>> {
    let light_green = Color(0.56, 0.93, 0.56);
    let white = Color::white();
    let stripe = Stripe::new(
        Box::new(Solid::new(light_green)),
        Box::new(Solid::new(white)),
    );

    let mut floor = Plane::new();

    let mut stripe_a = stripe.clone();
    stripe_a.set_transformation(rotation_y(PI / 4.0));

    let mut stripe_b = stripe;
    stripe_b.set_transformation(rotation_y(-PI / 4.0));

    let mut floor_material = Material::new();
    floor_material.pattern = Box::new(Blended::new(Box::new(stripe_a), Box::new(stripe_b)));
    floor.set_material(floor_material);

    let world = World {
        objects: vec![Box::new(floor)],
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0)),
    };

    let mut camera = Camera::new(150, 75, PI / 3.0);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);
    canvas.to_ppm("images/blended_pattern_floor.ppm")?;

    Ok(())
}
