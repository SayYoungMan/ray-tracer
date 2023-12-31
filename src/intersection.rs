use std::rc::Rc;

use crate::{
    constants::EPSILON,
    rays::Ray,
    shapes::{sphere::Sphere, Shape},
    tuples::{Point, Vector},
};

pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub reflectv: Vector,
    inside: bool,
    pub over_point: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Shape) -> Self {
        Intersection { t, object }
    }

    fn equals(&self, other: &Intersection) -> bool {
        self.t == other.t && self.object.equals(other.object)
    }

    pub fn prepare_computations(&self, ray: Ray) -> Computations<'a> {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point);
        let inside: bool;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        // Bump the point just a bit to make sure the intersection does not hide
        // behind the surface due to floating number errors
        let over_point = point + normalv * EPSILON;

        let reflectv = ray.direction.reflect(normalv);

        Computations {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv,
            reflectv,
            inside,
            over_point,
        }
    }
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let lowest_non_negative_t = intersections
        .into_iter()
        .filter(|int| int.t >= 0.0)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    lowest_non_negative_t
}

#[cfg(test)]
mod tests {
    use crate::{constants::EPSILON, shapes::plane::Plane, transformation::translation};

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(i.object.equals(&s));
    }

    #[test]
    fn hit_when_all_intersections_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];

        let i = hit(xs).unwrap();

        assert!(i.equals(&i1));
    }

    #[test]
    fn hit_when_some_intersections_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2];

        let i = hit(xs).unwrap();

        assert!(i.equals(&i2));
    }

    #[test]
    fn hit_when_all_intersections_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i1, i2];

        let i = hit(xs);

        assert!(i.is_none());
    }

    #[test]
    fn hit_is_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];

        let i = hit(xs).unwrap();

        assert!(i.equals(&i4));
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };

        let comps = i.prepare_computations(r);

        assert_eq!(comps.t, i.t);
        assert!(comps.object.equals(i.object));
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn precomputing_reflection_vector() {
        let shape = Plane::new();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);

        let comps = i.prepare_computations(r);

        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        )
    }

    #[test]
    fn hit_when_intersection_occurs_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };

        let comps = i.prepare_computations(r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_inside() {
        let r = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection {
            t: 1.0,
            object: &shape,
        };

        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        // Normal is inverted
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::new();
        shape.set_transformation(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);

        let comps = i.prepare_computations(r);

        assert!(comps.over_point.2 < -EPSILON / 2.0);
        assert!(comps.point.2 > comps.over_point.2);
    }
}
