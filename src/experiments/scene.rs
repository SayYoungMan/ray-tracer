use std::{error::Error, f64::consts::PI};

use crate::{
    camera::{self, Camera},
    color::Color,
    lights::PointLight,
    materials::Material,
    patterns::solid::Solid,
    shapes::{plane::Plane, sphere::Sphere, Shape},
    transformation::{rotation_x, rotation_y, scaling, translation, view_transform},
    tuples::{Point, Vector},
    world::{self, World},
};

pub fn draw_scene() -> Result<(), Box<dyn Error>> {
    let mut walls_material = Material::default();
    walls_material.pattern = Box::new(Solid::new(Color(1.0, 0.9, 0.9)));
    walls_material.specular = 0.0;

    // The floor is an extremely flattened sphere with a matte texture
    let mut floor = Sphere::new();
    floor.transformation = scaling(10.0, 0.01, 10.0);
    floor.material = walls_material.clone();

    // The wall on the left has the same scale and color as the floor but is also rotated and translated into place
    let mut left_wall = Sphere::new();
    left_wall.transformation = translation(0.0, 0.0, 5.0)
        * rotation_y(-PI / 4.0)
        * rotation_x(PI / 2.0)
        * scaling(10.0, 0.01, 10.0);
    left_wall.material = walls_material.clone();

    // The wall on the right is identical to left wall but is rotated the opposite direction in y
    let mut right_wall = Sphere::new();
    right_wall.transformation = translation(0.0, 0.0, 5.0)
        * rotation_y(PI / 4.0)
        * rotation_x(PI / 2.0)
        * scaling(10.0, 0.01, 10.0);
    right_wall.material = walls_material;

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green
    let mut middle = Sphere::new();
    middle.transformation = translation(-0.5, 1.0, 0.5);
    middle.material.pattern = Box::new(Solid::new(Color(0.1, 1.0, 0.5)));
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // The smaller green sphere on the right is scaled in half
    let mut right = Sphere::new();
    right.transformation = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material.pattern = Box::new(Solid::new(Color(0.5, 1.0, 0.1)));
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // The smallest sphere is scaled by thried, before being translated
    let mut left = Sphere::new();
    left.transformation = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.pattern = Box::new(Solid::new(Color(1.0, 0.8, 0.1)));
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    // The light source is white, shining from above and to the left
    let world = World {
        objects: vec![
            Box::new(floor),
            Box::new(left_wall),
            Box::new(right_wall),
            Box::new(middle),
            Box::new(right),
            Box::new(left),
        ],
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white()),
    };

    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);
    canvas.to_ppm("images/scene.ppm")?;

    Ok(())
}

pub fn draw_scene_with_plane() -> Result<(), Box<dyn Error>> {
    let plane = Plane::new();

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green
    let mut middle = Sphere::new();
    middle.transformation = translation(-0.5, 1.0, 0.5);
    middle.material.pattern = Box::new(Solid::new(Color(0.1, 1.0, 0.5)));
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // The smaller green sphere on the right is scaled in half
    let mut right = Sphere::new();
    right.transformation = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material.pattern = Box::new(Solid::new(Color(0.5, 1.0, 0.1)));
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // The smallest sphere is scaled by thried, before being translated
    let mut left = Sphere::new();
    left.transformation = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.pattern = Box::new(Solid::new(Color(1.0, 0.8, 0.1)));
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    // The light source is white, shining from above and to the left
    let world = World {
        objects: vec![
            Box::new(plane),
            Box::new(middle),
            Box::new(right),
            Box::new(left),
        ],
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white()),
    };

    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);
    canvas.to_ppm("images/scene_with_plane.ppm")?;

    Ok(())
}
