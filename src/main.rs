use std::error::Error;

use canvas::Canvas;
use projectile::{tick_until_fallen, Environment, Projectile};
use tuples::{new_point, new_vector};

use crate::{matrices::Matrix, tuples::SpatialTuple};

mod canvas;
mod color;
mod constants;
mod matrices;
mod projectile;
mod tuples;

fn draw_projectile() -> Result<(), Box<dyn Error>> {
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

#[allow(non_snake_case, unused)]
fn matrix_experiments() -> Result<(), Box<dyn Error>> {
    // It stays as identity matrix (I * I.inverse() = I)
    println!("Inverse of identity: {:?}", Matrix::identity().inverse());

    let A = Matrix::from_vec(vec![
        vec![0.0, 9.0, 3.0, 0.0],
        vec![9.0, 8.0, 0.0, 8.0],
        vec![1.0, 8.0, 5.0, 3.0],
        vec![0.0, 0.0, 5.0, 8.0],
    ]);
    println!("Original matrix: {:?}", A);
    // It will return identity matrix (A * A.inverse() = I)
    println!(
        "Multiply matrix by its inverse: {:?}",
        A.clone() * A.inverse()
    );

    let A = Matrix::from_vec(vec![
        vec![0.0, 9.0, 3.0, 0.0],
        vec![9.0, 8.0, 0.0, 8.0],
        vec![1.0, 8.0, 5.0, 3.0],
        vec![0.0, 0.0, 5.0, 8.0],
    ]);
    // They are equal
    println!(
        "Inverse of transpose: {:?}",
        A.clone().transpose().inverse()
    );
    println!("Transpose of inverse: {:?}", A.inverse().transpose());

    let A = Matrix::from_vec(vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);
    let b = SpatialTuple(1.0, 2.0, 3.0, 1.0);
    // This doubles only the second element of the tuple
    println!("Second of identity matrix is 2: {:?}", A * b);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // draw_projectile()
    matrix_experiments()
}
