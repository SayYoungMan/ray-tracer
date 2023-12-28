use crate::{
    intersection::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Vector},
};
use std::{any::Any, fmt::Debug};

pub mod plane;
pub mod sphere;

pub trait Shape: Debug {
    fn as_any(&self) -> &dyn Any;

    fn equals(&self, other: &dyn Shape) -> bool;

    fn material(&self) -> Material;

    fn set_material(&mut self, m: Material);

    fn transformation(&self) -> Matrix;

    fn set_transformation(&mut self, m: Matrix);

    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transformation().inverse());

        self.local_intersect(local_ray)
    }

    fn local_normal_at(&self, local_point: Point) -> Vector;

    fn normal_at(&self, point: Point) -> Vector {
        let local_point = self.transformation().inverse() * point;
        let local_normal = self.local_normal_at(local_point);

        let mut world_normal = self.transformation().inverse().transpose() * local_normal;
        world_normal.3 = 0.0;

        world_normal.normalize()
    }
}

#[derive(Debug)]
struct TestShape {
    transformation: Matrix,
    material: Material,
}

impl Shape for TestShape {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Shape) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<TestShape>() {
            self.transformation == other.transformation && self.material == other.material
        } else {
            false
        }
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.transformation = m;
    }

    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        panic!("{:?}", local_ray);
    }

    fn local_normal_at(&self, local_point: Point) -> Vector {
        Vector::new(local_point.0, local_point.1, local_point.2)
    }
}

impl TestShape {
    fn new() -> Self {
        Self {
            transformation: Matrix::identity(),
            material: Material::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        transformation::{rotation_z, scaling, translation},
        tuples::{Point, Vector},
    };

    use super::*;

    // Some basic default tests for all types implementing Shape
    // Please copy and paste this test for all shapes
    mod shape_default_tests {
        use crate::transformation::translation;

        use super::*;

        #[test]
        fn default_transformation() {
            let s = TestShape::new();
            assert_eq!(s.transformation, Matrix::identity());
        }

        #[test]
        fn assigning_transformation() {
            let mut s = TestShape::new();
            s.set_transformation(translation(2.0, 3.0, 4.0));
            assert_eq!(s.transformation, translation(2.0, 3.0, 4.0));
        }

        #[test]
        fn default_material() {
            let s = TestShape::new();
            assert_eq!(s.material, Material::new());
        }

        #[test]
        fn assigning_material() {
            let mut s = TestShape::new();

            let mut m = Material::new();
            m.ambient = 1.0;

            s.set_material(m.clone());

            assert_eq!(s.material, m);
        }
    }

    #[test]
    #[should_panic(
        expected = "Ray { origin: Point(0.0, 0.0, -2.5, 1.0), direction: Vector(0.0, 0.0, 0.5, 0.0) }"
    )]
    fn intersecting_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = TestShape::new();

        s.set_transformation(scaling(2.0, 2.0, 2.0));
        s.intersect(r);
    }

    #[test]
    #[should_panic(
        expected = "Ray { origin: Point(-5.0, 0.0, -5.0, 1.0), direction: Vector(0.0, 0.0, 1.0, 0.0) }"
    )]
    fn intersecting_translated_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = TestShape::new();

        s.set_transformation(translation(5.0, 0.0, 0.0));
        s.intersect(r);
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let mut s = TestShape::new();

        s.set_transformation(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_shape() {
        let mut s = TestShape::new();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);

        s.set_transformation(m);
        let n = s.normal_at(Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
