use std::error::Error;

use crate::{
    canvas::Canvas, color::Color, intersection::hit, rays::Ray, sphere::Sphere, tuples::Point,
};

pub fn draw_circle() -> Result<(), Box<dyn Error>> {
    const CANVAS_PIXELS: usize = 100;
    const WALL_SIZE: f64 = 7.0;
    const WALL_Z: f64 = 10.0;

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let pixel_size = WALL_SIZE / CANVAS_PIXELS as f64;
    let half = WALL_SIZE / 2.0;

    let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);
    let red = Color(1.0, 0.0, 0.0);
    let sphere = Sphere::new();

    // For each row of pixels in the canvas
    for y in 0..canvas.height {
        // Compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f64;

        // For each pixel in the row
        for x in 0..canvas.width {
            let world_x = -half + pixel_size * x as f64;

            // Describe the point on the wall that the ray will target
            let position = Point::new(world_x, world_y, WALL_Z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = sphere.intersect(r);

            if hit(xs).is_some() {
                canvas.write_pixel(x, y, red);
            }
        }
    }

    canvas.to_ppm("./images/circle.ppm")?;

    Ok(())
}
