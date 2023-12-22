use crate::sphere::Sphere;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Intersection { t, object }
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
    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::origin_unit_sphere();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn hit_when_all_intersections_positive_t() {
        let s = Sphere::origin_unit_sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];

        let i = hit(xs).unwrap();

        assert_eq!(i, i1);
    }

    #[test]
    fn hit_when_some_intersections_negative_t() {
        let s = Sphere::origin_unit_sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2];

        let i = hit(xs).unwrap();

        assert_eq!(i, i2);
    }

    #[test]
    fn hit_when_all_intersections_negative_t() {
        let s = Sphere::origin_unit_sphere();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i1, i2];

        let i = hit(xs);

        assert!(i.is_none());
    }

    #[test]
    fn hit_is_the_lowest_nonnegative_intersection() {
        let s = Sphere::origin_unit_sphere();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];

        let i = hit(xs).unwrap();

        assert_eq!(i, i4);
    }
}
