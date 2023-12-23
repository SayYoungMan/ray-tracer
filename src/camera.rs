use crate::{canvas::Canvas, matrices::Matrix, rays::Ray, tuples::Point, world::World};

pub struct Camera {
    // Horizontal size, in pixels, of the canvas that the picture will be rendered to
    hsize: usize,
    // Cavas's vertical size in pixels
    vsize: usize,
    // An angle that describes how much the camera can see
    field_of_view: f64,
    // Matrix describing how the world should be oriented relative to camera
    pub transform: Matrix,

    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    fn ray_for_pixel(&self, px: f64, py: f64) -> Ray {
        // The offset from the edge of the canvas to the pixel's center
        let x_offset = (px + 0.5) * self.pixel_size;
        let y_offset = (py + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space
        // (Camera looks towards -z, so +x is the LEFT)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // Using the camera matrix, transform the canvas point and the origin
        // then compute the ray's direction vector
        // (The canvas is at z = -1 for camera)
        let pixel = self.transform.inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Point::origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x as f64, y as f64);
                let color = world.color_at(ray);

                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        color::Color,
        constants::EPSILON,
        transformation::{view_transform, Transformation},
        tuples::{Point, Vector},
    };

    use super::*;

    #[test]
    fn constructing_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn pixel_size_for_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);

        let c = Camera::new(125, 200, PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100.0, 50.0);

        assert_eq!(r.origin, Point::origin());
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0.0, 0.0);

        assert_eq!(r.origin, Point::origin());
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_when_camera_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Transformation::RotationY(PI / 4.0).matrix()
            * Transformation::Translation(0.0, -2.0, 5.0).matrix();

        let r = c.ray_for_pixel(100.0, 50.0);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::origin();
        let up = Vector::new(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);

        let image = c.render(w);

        assert_eq!(image.pixel_at(5, 5), Color(0.38066, 0.47583, 0.2855));
    }
}
