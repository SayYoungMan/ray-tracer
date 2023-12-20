use std::error::Error;

use canvas::Canvas;
use projectile::{tick_until_fallen, Environment, Projectile};
use tuples::{new_point, new_vector};

mod canvas;
mod color;
mod constants;
mod matrices;
mod projectile;
mod tuples;

fn main() -> Result<(), Box<dyn Error>> {
    let start = new_point(0.0, 1.0, 0.0);
    let velocity = new_vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile {
        position: start,
        velocity,
    };

    let gravity = new_vector(0.0, -0.1, 0.0);
    let wind = new_vector(-0.01, 0.0, 0.0);
    let e = Environment { gravity, wind };

    let mut c = Canvas::new(900, 550);

    tick_until_fallen(&mut c, &e, &mut p);

    if let Err(err) = c.to_ppm("output.ppm") {
        eprintln!("Error occurred while generating PPM file: {}", err);
        return Err(err.into());
    }

    Ok(())
}
