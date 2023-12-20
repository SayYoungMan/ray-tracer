use crate::{sphere::Sphere, tuples::SpatialTuple};

pub struct Ray {
    origin: SpatialTuple,
    direction: SpatialTuple,
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

    pub fn intersect_sphere(&self, sphere: Sphere) -> Option<(f64, f64)> {
        // Vector from the sphere's center to the ray origin
        let sphere_to_ray = self.origin - sphere.center;

        let a = self.direction.dot(&self.direction);
        let b = 2.0 * self.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - sphere.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Some((t1, t2))
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
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = r.intersect_sphere(s).unwrap();

        assert_eq!(xs.0, 4.0);
        assert_eq!(xs.1, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(new_point(0.0, 1.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = r.intersect_sphere(s).unwrap();

        assert_eq!(xs.0, 5.0);
        assert_eq!(xs.1, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(new_point(0.0, 2.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = r.intersect_sphere(s);

        assert!(xs.is_none());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = r.intersect_sphere(s).unwrap();

        assert_eq!(xs.0, -1.0);
        assert_eq!(xs.1, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = r.intersect_sphere(s).unwrap();

        assert_eq!(xs.0, -6.0);
        assert_eq!(xs.1, -4.0);
    }
}
