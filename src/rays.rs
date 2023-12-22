use crate::{sphere::Sphere, transformation::Transformation, tuples::SpatialTuple};

#[derive(Debug)]
pub struct Ray {
    pub origin: SpatialTuple,
    pub direction: SpatialTuple,
}

impl Ray {
    pub fn new(origin: SpatialTuple, direction: SpatialTuple) -> Self {
        if origin.3 != 1.0 || direction.3 != 0.0 {
            panic!("The origin of ray should be a point and direction should be a vector. Received origin: {:#?} and direction: {:#?}", origin, direction)
        }

        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> SpatialTuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Transformation) -> Self {
        match m {
            Transformation::Translation(x, y, z) => Ray {
                origin: self.origin.inverse_translate(x, y, z),
                direction: self.direction,
            },
            Transformation::Scaling(x, y, z) => Ray {
                origin: self.origin.inverse_scale(x, y, z),
                direction: self.direction.inverse_scale(x, y, z),
            },
            Transformation::RotationX(r) => Ray {
                origin: self.origin,
                direction: self.direction.inverse_rotate_x(r),
            },
            Transformation::RotationY(r) => Ray {
                origin: self.origin,
                direction: self.direction.inverse_rotate_y(r),
            },
            Transformation::RotationZ(r) => Ray {
                origin: self.origin,
                direction: self.direction.inverse_rotate_z(r),
            },
            Transformation::Shearing(x_y, x_z, y_x, y_z, z_x, z_y) => Ray {
                origin: self.origin,
                direction: self.direction.inverse_shear(x_y, x_z, y_x, y_z, z_x, z_y),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::{new_point, new_vector};

    use super::*;

    #[test]
    fn creating_and_querying_ray() {
        let origin = new_point(1.0, 2.0, 3.0);
        let direction = new_vector(4.0, 5.0, 6.0);

        let r = Ray { origin, direction };

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray {
            origin: new_point(2.0, 3.0, 4.0),
            direction: new_vector(1.0, 0.0, 0.0),
        };

        assert_eq!(r.position(0.0), new_point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), new_point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), new_point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_ray() {
        let r = Ray::new(new_point(1.0, 2.0, 3.0), new_vector(0.0, 1.0, 0.0));
        let m = Transformation::Translation(3.0, 4.0, 5.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, new_point(-2.0, -2.0, -2.0));
        assert_eq!(r2.direction, new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_ray() {
        let r = Ray::new(new_point(1.0, 2.0, 3.0), new_vector(0.0, 1.0, 0.0));
        let m = Transformation::Scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, new_point(0.5, 0.66667, 0.75));
        assert_eq!(r2.direction, new_vector(0.0, 0.33333, 0.0));
    }
}
