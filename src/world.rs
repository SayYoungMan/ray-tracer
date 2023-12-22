use crate::{
    color::Color, lights::PointLight, sphere::Sphere, transformation::Transformation, tuples::Point,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn creating_world() {
        let w = World::new();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light.intensity, Color(0.0, 0.0, 0.0));
    }

    fn default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0));

        let mut s1 = Sphere::origin_unit_sphere();
        s1.material.color = Color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::origin_unit_sphere();
        s2.set_transformation(vec![Transformation::Scaling(0.5, 0.5, 0.5)]);

        let w = World::default();

        assert_eq!(w.light, light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }
}
