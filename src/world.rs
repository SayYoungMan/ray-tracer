use crate::{
    color::Color,
    intersection::{hit, Computations, Intersection},
    lights::PointLight,
    rays::Ray,
    sphere::Sphere,
    transformation::Transformation,
    tuples::Point,
};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light: PointLight,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: PointLight::new(Point::origin(), Color(0.0, 0.0, 0.0)),
        }
    }

    pub fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0));

        let mut s1 = Sphere::new();
        s1.material.color = Color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.set_transformation(vec![Transformation::Scaling(0.5, 0.5, 0.5)]);

        Self {
            objects: vec![s1, s2],
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

    fn shade_hit(&self, comps: Computations) -> Color {
        comps
            .object
            .material
            .lighting(&self.light, comps.point, comps.eyev, comps.normalv)
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let intersections = self.intersect(r);
        let hit = hit(intersections);

        if hit.is_none() {
            return Color(0.0, 0.0, 0.0);
        }

        let comps = hit.unwrap().prepare_computations(r);

        self.shade_hit(comps)
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

    #[test]
    fn shading_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection {
            t: 4.0,
            object: shape,
        };

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default();
        w.light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color(1.0, 1.0, 1.0));
        let r = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection {
            t: 0.5,
            object: shape,
        };

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));

        let c = w.color_at(r);

        assert_eq!(c, Color(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let c = w.color_at(r);

        assert_eq!(c, Color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let c = w.color_at(r);

        assert_eq!(c, w.objects[1].material.color);
    }
}
