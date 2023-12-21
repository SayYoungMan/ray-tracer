use uuid::Uuid;

use crate::{
    intersection::Intersection,
    rays::Ray,
    tuples::{new_point, SpatialTuple},
};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: Uuid,
    pub center: SpatialTuple,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: SpatialTuple, radius: f64) -> Self {
        Sphere {
            id: Uuid::new_v4(),
            center,
            radius,
        }
    }

    pub fn origin_unit_sphere() -> Self {
        Sphere::new(new_point(0.0, 0.0, 0.0), 1.0)
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let vec = Vec::new();

        // Vector from the sphere's center to the ray origin
        let sphere_to_ray = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, &self), Intersection::new(t2, &self)]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        rays::Ray,
        tuples::{new_point, new_vector},
    };

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(new_point(0.0, 1.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(new_point(0.0, 2.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::origin_unit_sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }
}
