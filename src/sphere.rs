use uuid::Uuid;

use crate::{
    constants::EPSILON,
    intersection::Intersection,
    materials::Material,
    rays::Ray,
    transformation::Transformation,
    tuples::{Point, Vector},
};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: Uuid,
    pub transformations: Vec<Transformation>,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            id: Uuid::new_v4(),
            transformations: Vec::new(),
            material: Material::default(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut new_ray = ray;
        for t in &self.transformations {
            new_ray = new_ray.transform(*t);
        }

        // Vector from the sphere's center to the ray origin
        let sphere_to_ray = new_ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = new_ray.direction.dot(&new_ray.direction);
        let b = 2.0 * new_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

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

    pub fn normal_at(&self, p: Point) -> Vector {
        let mut point = p;
        let mut normal = (p - Point::new(0.0, 0.0, 0.0)).normalize();
        for t in self.transformations.iter().rev() {
            point = t.matrix().inverse() * point;
            let object_normal = point - Point::new(0.0, 0.0, 0.0);

            normal = t.matrix().inverse().transpose() * object_normal;
            normal.3 = 0.0;
        }

        normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{color::Color, rays::Ray};

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::new();

        assert_eq!(s.transformations.len(), 0);

        let t = Transformation::Translation(2.0, 3.0, 4.0);

        s.set_transformation(vec![t]);

        assert_eq!(s.transformations.len(), 1);
        assert_eq!(s.transformations[0], t);
    }

    #[test]
    fn intersecting_scaled_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transformation(vec![Transformation::Scaling(2.0, 2.0, 2.0)]);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transformation(vec![Transformation::Translation(5.0, 0.0, 0.0)]);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_sphere() {
        let s = Sphere::new();

        // On the x-axis
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));

        // On the y-axis
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));

        // On the z-axis
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));

        // At non-axial point
        let n = s.normal_at(Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Vector::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            )
        );
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::new();

        let n = s.normal_at(Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transformation(vec![Transformation::Translation(0.0, 1.0, 0.0)]);

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        let m = vec![
            Transformation::Scaling(1.0, 0.5, 1.0),
            Transformation::RotationZ(PI / 0.5),
        ];
        s.set_transformation(m);

        let n = s.normal_at(Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();

        assert_eq!(s.material, Material::default());
    }
}
