use crate::matrices::Matrix;

pub struct Camera {
    // Horizontal size, in pixels, of the canvas that the picture will be rendered to
    hsize: usize,
    // Cavas's vertical size in pixels
    vsize: usize,
    // An angle that describes how much the camera can see
    field_of_view: f64,
    // Matrix describing how the world should be oriented relative to camera
    transform: Matrix,

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
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::constants::EPSILON;

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
}
