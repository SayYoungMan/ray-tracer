use std::error::Error;

use crate::tuples::{Point, Vector};
use crate::{canvas::Canvas, color::Color, tuples};

pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

pub fn tick_until_fallen(canvas: &mut Canvas, env: &Environment, proj: &mut Projectile) {
    let mut height = proj.position.1;

    while height > 0.0 {
        tick(env, proj);
        canvas.write_pixel(
            proj.position.0 as usize,
            canvas.height - proj.position.1 as usize,
            Color::white(),
        );
        height = proj.position.1;
    }
}

pub fn draw_projectile() -> Result<(), Box<dyn Error>> {
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile {
        position: start,
        velocity,
    };

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let e = Environment { gravity, wind };

    let mut c = Canvas::new(900, 550);

    tick_until_fallen(&mut c, &e, &mut p);

    c.to_ppm("projectile.ppm")?;

    Ok(())
}
