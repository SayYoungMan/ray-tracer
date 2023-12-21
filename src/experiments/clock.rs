use std::{error::Error, f64::consts::PI};

use crate::{
    canvas::Canvas,
    color::Color,
    tuples::{new_point, SpatialTuple},
};

const ARM_LENGTH: f64 = 37.5;
const MID_POINT: f64 = 50.0;
const FULL_LENGTH: f64 = 100.0;

pub fn draw_clock() -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new(FULL_LENGTH as usize, FULL_LENGTH as usize);
    let twelve_oclock = new_point(0.0, 0.0, ARM_LENGTH);
    draw_point(&mut canvas, &twelve_oclock);

    for i in 1..12 {
        draw_point(&mut canvas, &twelve_oclock.rotate_y(i as f64 * (PI / 6.0)))
    }

    canvas.to_ppm("clock.ppm")?;

    Ok(())
}

fn draw_point(canvas: &mut Canvas, point: &SpatialTuple) {
    canvas.write_pixel(
        (MID_POINT + point.2) as usize,
        (MID_POINT - point.0) as usize,
        Color(1.0, 1.0, 1.0),
    );
}
