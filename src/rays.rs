use crate::{sphere::Sphere, tuples::SpatialTuple};

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
}
