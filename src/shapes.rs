use crate::{
    intersection::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Vector},
};

pub mod sphere;

pub trait Shape {
    fn new() -> Self;

    fn transformation(&self) -> Matrix;

    fn set_transformation(&mut self, m: Matrix);

    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transformation().inverse());

        self.local_intersect(local_ray)
    }
}

struct TestShape {
    transformation: Matrix,
    material: Material,
}

impl Shape for TestShape {
    fn new() -> Self {
        Self {
            transformation: Matrix::identity(),
            material: Material::default(),
        }
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
}

#[cfg(test)]
mod tests {
    use crate::{
        transformation::{scaling, translation},
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
            assert_eq!(s.material, Material::default());
        }

        #[test]
        fn assigning_material() {
            let mut s = TestShape::new();

            let mut m = Material::default();
            m.ambient = 1.0;

            s.material = m;

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
}
