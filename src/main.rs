use std::error::Error;

use clock::draw_clock;
use matrices::matrix_experiments;
use projectile::draw_projectile;

mod canvas;
mod clock;
mod color;
mod constants;
mod matrices;
mod projectile;
mod rays;
mod transformation;
mod tuples;

fn main() -> Result<(), Box<dyn Error>> {
    // draw_projectile()
    // matrix_experiments()
    draw_clock()
}
