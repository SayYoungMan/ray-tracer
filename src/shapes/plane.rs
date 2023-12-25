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

#[derive(Debug)]
pub struct Plane {
    pub transformation: Matrix,
    pub material: Material,
}

impl Shape for Plane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Shape) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Plane>() {
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
        if local_ray.direction.1.abs() < EPSILON {
            return Vec::new();
        }

        let t = -local_ray.origin.1 / local_ray.direction.1;
        vec![Intersection::new(t, self)]
    }

    fn local_normal_at(&self, _local_point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

impl Plane {
    pub fn new() -> Self {
        Self {
            transformation: Matrix::identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::{Point, Vector};

    use super::*;

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = Plane::new();

        let n1 = p.local_normal_at(Point::origin());
        let n2 = p.local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));

        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));

        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(xs[0].object.equals(&p));
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let p = Plane::new();

        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(xs[0].object.equals(&p));
    }
}
