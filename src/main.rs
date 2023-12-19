use canvas::Canvas;
use color::Color;
use projectile::tick_until_fallen;
use tuples::{new_point, new_vector};

mod canvas;
mod color;
mod constants;
mod projectile;
mod tuples;

fn main() {
    // let mut p = projectile::Projectile {
    //     // Projectile starts one unit above the origin
    //     position: new_point(0.0, 1.0, 0.0),
    //     // Velocity is normalized to 1 unit/tick
    //     velocity: new_vector(1.0, 1.0, 0.0).normalize(),
    // };

    // let e = projectile::Environment {
    //     gravity: new_vector(0.0, -0.1, 0.0),
    //     wind: new_vector(-0.01, 0.0, 0.0),
    // };

    // tick_until_fallen(&e, &mut p);

    let canvas = Canvas::with_filled_color(50, 50, Color(1.0, 0.0, 1.0));
    canvas.to_ppm("./output.ppm");
}
