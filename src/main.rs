#![allow(dead_code, unused_imports)]

use std::error::Error;

mod canvas;
mod color;
mod constants;
mod experiments;
mod intersection;
mod lights;
mod materials;
mod matrices;
mod rays;
mod sphere;
mod transformation;
mod tuples;

fn main() -> Result<(), Box<dyn Error>> {
    // experiments::projectile::draw_projectile()
    // experiments::matrix::matrix_experiments()
    // experiments::clock::draw_clock()
    experiments::circle::draw_circle()
}
