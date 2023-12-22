use crate::{
    color::Color, intersection::Intersection, lights::PointLight, rays::Ray, sphere::Sphere,
    transformation::Transformation, tuples::Point,
};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light: PointLight,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: PointLight::new(Point::new(0.0, 0.0, 0.0), Color(0.0, 0.0, 0.0)),
        }
    }

    pub fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0));

        let mut s1 = Sphere::origin_unit_sphere();
        s1.material.color = Color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::origin_unit_sphere();
        s2.set_transformation(vec![Transformation::Scaling(0.5, 0.5, 0.5)]);

        Self {
            objects: vec![s1, s2],
            light,
        }
    }

    pub fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let mut xs = Vec::new();
        for object in self.objects.iter() {
            xs.append(&mut object.intersect(r));
        }

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::Vector;

    use super::*;

    #[test]
    fn creating_world() {
        let w = World::new();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light.intensity, Color(0.0, 0.0, 0.0));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}
