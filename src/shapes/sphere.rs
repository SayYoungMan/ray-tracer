use std::any::Any;

use crate::{
    constants::EPSILON,
    intersection::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Vector},
};

use super::Shape;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub transformation: Matrix,
    pub material: Material,
}

impl Shape for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Shape) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Sphere>() {
            self.transformation == other.transformation && self.material == other.material
        } else {
            false
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.transformation = m;
    }

    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        // Vector from the sphere's center to the ray origin
        let sphere_to_ray = local_ray.origin - Point::origin();

        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, &self), Intersection::new(t2, &self)]
    }

    fn local_normal_at(&self, local_point: Point) -> Vector {
        Vector::new(local_point.0, local_point.1, local_point.2)
    }
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transformation: Matrix::identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        color::Color,
        rays::Ray,
        transformation::{rotation_z, scaling, translation},
    };

    use super::*;

    mod shape_default_tests {
        use crate::transformation::translation;

        use super::*;

        #[test]
        fn default_transformation() {
            let s = Sphere::new();
            assert_eq!(s.transformation, Matrix::identity());
        }

        #[test]
        fn assigning_transformation() {
            let mut s = Sphere::new();
            s.set_transformation(translation(2.0, 3.0, 4.0));
            assert_eq!(s.transformation, translation(2.0, 3.0, 4.0));
        }

        #[test]
        fn default_material() {
            let s = Sphere::new();
            assert_eq!(s.material, Material::default());
        }

        #[test]
        fn assigning_material() {
            let mut s = Sphere::new();

            let mut m = Material::default();
            m.ambient = 1.0;

            s.material = m;

            assert_eq!(s.material, m);
        }
    }

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
        let r = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
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
        assert!(xs[0].object.equals(&s));
        assert!(xs[1].object.equals(&s));
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::new();

        assert_eq!(s.transformation, Matrix::identity());

        let t = translation(2.0, 3.0, 4.0);

        s.set_transformation(t.clone());

        assert_eq!(s.transformation, t);
    }

    #[test]
    fn intersecting_scaled_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transformation(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transformation(translation(5.0, 0.0, 0.0));
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
        s.set_transformation(translation(0.0, 1.0, 0.0));

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
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
