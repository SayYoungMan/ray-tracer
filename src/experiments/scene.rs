use std::{error::Error, f64::consts::PI};

use crate::{
    camera::{self, Camera},
    color::Color,
    lights::PointLight,
    materials::Material,
    sphere::Sphere,
    transformation::{view_transform, Transformation},
    tuples::{Point, Vector},
    world::{self, World},
};

pub fn draw_scene() -> Result<(), Box<dyn Error>> {
    let mut walls_material = Material::default();
    walls_material.color = Color(1.0, 0.9, 0.9);
    walls_material.specular = 0.0;

    // The floor is an extremely flattened sphere with a matte texture
    let mut floor = Sphere::new();
    floor.transformations = vec![Transformation::Scaling(10.0, 0.01, 10.0)];
    floor.material = walls_material;

    // The wall on the left has the same scale and color as the floor but is also rotated and translated into place
    let mut left_wall = Sphere::new();
    left_wall.transformations = vec![
        Transformation::Translation(0.0, 0.0, 5.0),
        Transformation::RotationY(-PI / 4.0),
        Transformation::RotationX(PI / 2.0),
        Transformation::Scaling(10.0, 0.01, 10.0),
    ];
    left_wall.material = walls_material;

    // The wall on the right is identical to left wall but is rotated the opposite direction in y
    let mut right_wall = Sphere::new();
    right_wall.transformations = vec![
        Transformation::Translation(0.0, 0.0, 5.0),
        Transformation::RotationY(PI / 4.0),
        Transformation::RotationX(PI / 2.0),
        Transformation::Scaling(10.0, 0.01, 10.0),
    ];
    right_wall.material = walls_material;

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green
    let mut middle = Sphere::new();
    middle.transformations = vec![Transformation::Translation(-0.5, 1.0, 0.5)];
    middle.material.color = Color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // The smaller green sphere on the right is scaled in half
    let mut right = Sphere::new();
    right.transformations = vec![
        Transformation::Translation(1.5, 0.5, -0.5),
        Transformation::Scaling(0.5, 0.5, 0.5),
    ];
    right.material.color = Color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // The smallest sphere is scaled by thried, before being translated
    let mut left = Sphere::new();
    left.transformations = vec![
        Transformation::Translation(-1.5, 0.33, -0.75),
        Transformation::Scaling(0.33, 0.33, 0.33),
    ];
    left.material.color = Color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    // The light source is white, shining from above and to the left
    let world = World {
        objects: vec![floor, left_wall, right_wall, middle, right, left],
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0)),
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
