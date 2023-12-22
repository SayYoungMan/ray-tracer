use std::error::Error;

use crate::{
    canvas::Canvas, color::Color, intersection::hit, lights::PointLight, rays::Ray, sphere::Sphere,
    tuples::Point,
};

pub fn draw_sphere() -> Result<(), Box<dyn Error>> {
    const CANVAS_PIXELS: usize = 500;
    const WALL_SIZE: f64 = 7.0;
    const WALL_Z: f64 = 10.0;

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let pixel_size = WALL_SIZE / CANVAS_PIXELS as f64;
    let half = WALL_SIZE / 2.0;

    let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);

    let mut sphere = Sphere::origin_unit_sphere();
    sphere.material.color = Color(1.0, 0.2, 1.0);

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

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

            let hit = hit(xs);
            if hit.is_some() {
                let hit = hit.unwrap();
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -r.direction;

                let color = hit.object.material.lighting(&light, point, eye, normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    canvas.to_ppm("./images/sphere.ppm")?;

    Ok(())
}
