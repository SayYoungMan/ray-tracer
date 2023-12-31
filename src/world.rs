use crate::{
    color::Color,
    intersection::{hit, Computations, Intersection},
    lights::PointLight,
    patterns::solid::Solid,
    rays::Ray,
    shapes::{sphere::Sphere, Shape},
    transformation::scaling,
    tuples::Point,
};

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub light: PointLight,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: PointLight::new(Point::origin(), Color::black()),
        }
    }

    pub fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

        let mut s1 = Sphere::new();
        s1.material.pattern = Box::new(Solid::new(Color(0.8, 1.0, 0.6)));
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.set_transformation(scaling(0.5, 0.5, 0.5));

        Self {
            objects: vec![Box::new(s1), Box::new(s2)],
            light,
        }
    }

    fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let mut xs = Vec::new();
        for object in self.objects.iter() {
            xs.append(&mut object.intersect(r));
        }

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn is_shadowed(&self, point: Point) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect(r);

        let h = hit(intersections);

        match h {
            Some(h) => h.t < distance,
            None => false,
        }
    }

    fn shade_hit(&self, comps: Computations, remaining: usize) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);

        let surface = comps.object.material().lighting(
            &self.light,
            comps.point,
            comps.eyev,
            comps.normalv,
            shadowed,
            comps.object,
        );

        let reflected = self.reflected_color(comps, remaining);

        surface + reflected
    }

    pub fn color_at(&self, r: Ray, remaining: usize) -> Color {
        let intersections = self.intersect(r);
        let hit = hit(intersections);

        if hit.is_none() {
            return Color::black();
        }

        let comps = hit.unwrap().prepare_computations(r);

        self.shade_hit(comps, remaining)
    }

    fn reflected_color(&self, comps: Computations, remaining: usize) -> Color {
        if remaining <= 0 || comps.object.material().reflective == 0.0 {
            return Color::black();
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(reflect_ray, remaining - 1);

        color * comps.object.material().reflective
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::MAX_REFLECTION_DEPTH, materials::Material, shapes::plane::Plane,
        transformation::translation, tuples::Vector,
    };

    use super::*;

    #[test]
    fn creating_world() {
        let w = World::new();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light.intensity, Color::black());
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

    #[test]
    fn shading_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection {
            t: 4.0,
            object: shape.as_ref(),
        };

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps, MAX_REFLECTION_DEPTH);

        assert_eq!(c, Color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default();
        w.light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::white());
        let r = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection {
            t: 0.5,
            object: shape.as_ref(),
        };

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps, MAX_REFLECTION_DEPTH);

        assert_eq!(c, Color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));

        let c = w.color_at(r, MAX_REFLECTION_DEPTH);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let c = w.color_at(r, MAX_REFLECTION_DEPTH);

        assert_eq!(c, Color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default();

        let mut m = Material::new();
        m.ambient = 1.0;

        w.objects[1].set_material(m);

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let c = w.color_at(r, MAX_REFLECTION_DEPTH);

        assert_eq!(
            c,
            w.objects[1]
                .material()
                .pattern
                .at(Point::new(0.0, 0.0, 0.75))
        );
    }

    #[test]
    fn reflected_color_for_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));

        let shape = w.objects[1].as_mut();
        let mut material = Material::new();
        material.ambient = 1.0;
        shape.set_material(material);

        let i = Intersection::new(1.0, w.objects[1].as_ref());

        let comps = i.prepare_computations(r);
        let color = w.reflected_color(comps, MAX_REFLECTION_DEPTH);

        assert_eq!(color, Color::black());
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.set_transformation(translation(0.0, -1.0, 0.0));

        let mut w = World::default();
        w.objects.push(Box::new(shape));

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), w.objects[2].as_ref());

        let comps = i.prepare_computations(r);
        let color = w.reflected_color(comps, MAX_REFLECTION_DEPTH);

        assert_eq!(color, Color(0.19033, 0.23792, 0.14275));
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = World::default();

        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.set_transformation(translation(0.0, -1.0, 0.0));

        w.objects.push(Box::new(shape));

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), w.objects[2].as_ref());

        let comps = i.prepare_computations(r);
        let color = w.shade_hit(comps, MAX_REFLECTION_DEPTH);

        assert_eq!(color, Color(0.87676, 0.92434, 0.82917));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.light = PointLight::new(Point::origin(), Color::white());

        let mut lower = Plane::new();
        lower.material.reflective = 1.0;
        lower.transformation = translation(0.0, -1.0, 0.0);
        w.objects.push(Box::new(lower));

        let mut upper = Plane::new();
        upper.material.reflective = 1.0;
        upper.transformation = translation(0.0, 1.0, 0.0);
        w.objects.push(Box::new(upper));

        let r = Ray::new(Point::origin(), Vector::new(0.0, 1.0, 0.0));
        w.color_at(r, MAX_REFLECTION_DEPTH);
    }

    #[test]
    fn reflected_color_at_max_recursive_depth() {
        let mut w = World::default();

        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.set_transformation(translation(0.0, -1.0, 0.0));

        w.objects.push(Box::new(shape));

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), w.objects[2].as_ref());

        let comps = i.prepare_computations(r);
        let color = w.reflected_color(comps, 0);

        assert_eq!(color, Color::black());
    }

    mod shadow {
        use crate::transformation::translation;

        use super::*;

        #[test]
        fn no_shadow_when_nothing_collinear_with_point_and_light() {
            let w = World::default();
            let p = Point::new(0.0, 10.0, 0.0);

            assert_eq!(w.is_shadowed(p), false);
        }

        #[test]
        fn shadow_when_object_is_between_point_and_light() {
            let w = World::default();
            let p = Point::new(10.0, -10.0, 10.0);

            assert_eq!(w.is_shadowed(p), true);
        }

        #[test]
        fn no_shadow_when_object_behind_light() {
            let w = World::default();
            let p = Point::new(-20.0, 20.0, -20.0);

            assert_eq!(w.is_shadowed(p), false);
        }

        #[test]
        fn no_shadow_when_object_behind_point() {
            let w = World::default();
            let p = Point::new(-2.0, 2.0, -2.0);

            assert_eq!(w.is_shadowed(p), false);
        }

        #[test]
        fn shade_hit_is_given_intersection_in_shadow() {
            let mut w = World::default();
            w.light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());

            let s1 = Sphere::new();
            let mut s2 = Sphere::new();
            s2.set_transformation(translation(0.0, 0.0, 10.0));
            w.objects = vec![Box::new(s1), Box::new(s2.clone())];

            let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
            let i = Intersection::new(4.0, &s2);

            let comps = i.prepare_computations(r);
            let c = w.shade_hit(comps, MAX_REFLECTION_DEPTH);

            assert_eq!(c, Color(0.1, 0.1, 0.1));
        }
    }
}
