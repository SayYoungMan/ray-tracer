#![allow(dead_code, unused_imports)]

use std::error::Error;

mod camera;
mod canvas;
mod color;
mod constants;
mod experiments;
mod intersection;
mod lights;
mod materials;
mod matrices;
mod patterns;
mod rays;
mod shapes;
mod transformation;
mod tuples;
mod utils;
mod world;

fn main() -> Result<(), Box<dyn Error>> {
    // experiments::projectile::draw_projectile()
    // experiments::matrix::matrix_experiments()
    // experiments::clock::draw_clock()
    // experiments::circle::draw_circle()
    // experiments::sphere::draw_sphere()
    // experiments::scene::draw_scene()
    // experiments::scene::draw_scene_with_plane()
    experiments::pattern::blended_pattern_floor()
}
