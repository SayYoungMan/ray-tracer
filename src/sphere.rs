use uuid::Uuid;

use crate::{
    intersection::Intersection,
    rays::Ray,
    transformation::Transformation,
    tuples::{new_point, SpatialTuple},
};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: Uuid,
    pub center: SpatialTuple,
    pub radius: f64,
    pub transformations: Vec<Transformation>,
}

impl Sphere {
    pub fn new(center: SpatialTuple, radius: f64) -> Self {
        Sphere {
            id: Uuid::new_v4(),
            center,
            radius,
            transformations: Vec::new(),
        }
    }

    pub fn origin_unit_sphere() -> Self {
        Sphere::new(new_point(0.0, 0.0, 0.0), 1.0)
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut new_ray = ray;
        for t in &self.transformations {
            new_ray = new_ray.transform(*t);
        }

        // Vector from the sphere's center to the ray origin
        let sphere_to_ray = new_ray.origin - self.center;

        let a = new_ray.direction.dot(&new_ray.direction);
        let b = 2.0 * new_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, &self), Intersection::new(t2, &self)]
    }

    pub fn set_transformation(&mut self, m: Vec<Transformation>) {
        self.transformations = m;
    }

    fn is_point_on_surface(&self, p: SpatialTuple) -> bool {
        (p - self.center).magnitude() == self.radius
    }

    pub fn normal_at(&self, p: SpatialTuple) -> SpatialTuple {
        if !self.is_point_on_surface(p) {
            panic!(
                "Can't find the normal because the point {:?} is not on the surface of sphere {:#?}",
                p, self
            );
        }

        (p - self.center).normalize()
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

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::origin_unit_sphere();

        assert_eq!(s.transformations.len(), 0);

        let t = Transformation::Translation(2.0, 3.0, 4.0);

        s.set_transformation(vec![t]);

        assert_eq!(s.transformations.len(), 1);
        assert_eq!(s.transformations[0], t);
    }

    #[test]
    fn intersecting_scaled_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let mut s = Sphere::origin_unit_sphere();

        s.set_transformation(vec![Transformation::Scaling(2.0, 2.0, 2.0)]);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let mut s = Sphere::origin_unit_sphere();

        s.set_transformation(vec![Transformation::Translation(5.0, 0.0, 0.0)]);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_sphere() {
        let s = Sphere::origin_unit_sphere();

        // On the x-axis
        let n = s.normal_at(new_point(1.0, 0.0, 0.0));
        assert_eq!(n, new_vector(1.0, 0.0, 0.0));

        // On the y-axis
        let n = s.normal_at(new_point(0.0, 1.0, 0.0));
        assert_eq!(n, new_vector(0.0, 1.0, 0.0));

        // On the z-axis
        let n = s.normal_at(new_point(0.0, 0.0, 1.0));
        assert_eq!(n, new_vector(0.0, 0.0, 1.0));

        // At non-axial point
        let n = s.normal_at(new_point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            new_vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            )
        );
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::origin_unit_sphere();

        let n = s.normal_at(new_point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }
}
