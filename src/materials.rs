use crate::{
    color::Color,
    lights::PointLight,
    patterns::Pattern,
    shapes::Shape,
    tuples::{Point, Vector},
};

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            pattern: None,
        }
    }

    pub fn default() -> Self {
        Self::new(Color::white(), 0.1, 0.9, 0.9, 200.0)
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
        object: &dyn Shape,
    ) -> Color {
        let color = match &self.pattern {
            Some(pattern) => pattern.at_object(object, point),
            None => self.color,
        };

        // Combine the surface color with the light's color/intensity
        let effective_color = color * light.intensity;

        // Find the direction to the light source
        let lightv = (light.position - point).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot(&normalv);
        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0.0 || in_shadow {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Self {
            color: self.color.clone(),
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
            pattern: self.pattern.as_ref().map(|p| p.clone_box()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.color, Color::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    mod lighting {
        use super::*;
        use crate::{lights::PointLight, patterns::Stripe, shapes::sphere::Sphere};

        const M: Material = Material {
            color: Color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        };
        const POSITION: Point = Point(0.0, 0.0, 0.0, 1.0);

        #[test]
        fn lighting_with_the_surface_in_shadow() {
            // Ambient, diffuse, and specular all at full strength
            let eyev = Vector::new(0.0, 0.0, -1.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
            let sphere = Sphere::new();

            let result = M.lighting(&light, POSITION, eyev, normalv, false, &sphere);

            assert_eq!(result, Color(1.9, 1.9, 1.9));
        }

        #[test]
        fn lighting_with_eye_between_light_and_surface_with_eye_offset_45deg() {
            // Ambient and diffuse should still be full strength because the light and normal vectors are the same
            // Specular value have fallen off to 0
            let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
            let sphere = Sphere::new();

            let result = M.lighting(&light, POSITION, eyev, normalv, true, &sphere);

            assert_eq!(result, Color(0.1, 0.1, 0.1));
        }

        #[test]
        fn lighting_with_eye_opposite_surface_with_light_offset_45deg() {
            // Angle between light and normal vectors changed so diffuse changes
            // Specular component falls off to 0 as well
            let eyev = Vector::new(0.0, 0.0, -1.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
            let sphere = Sphere::new();

            let result = M.lighting(&light, POSITION, eyev, normalv, false, &sphere);

            assert_eq!(result, Color(0.7364, 0.7364, 0.7364));
        }

        #[test]
        fn lighting_with_eye_in_path_of_reflection_vector() {
            // Diffuse is the same as before but specular is at full strength
            let eyev = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
            let sphere = Sphere::new();

            let result = M.lighting(&light, POSITION, eyev, normalv, false, &sphere);

            assert_eq!(result, Color(1.6364, 1.6364, 1.6364));
        }

        #[test]
        fn lighting_with_light_behind_the_surface() {
            // Light no longer illuminates the surface, so the diffuse and specular go to 0
            let eyev = Vector::new(0.0, 0.0, -1.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::white());
            let sphere = Sphere::new();

            let result = M.lighting(&light, POSITION, eyev, normalv, false, &sphere);

            assert_eq!(result, Color(0.1, 0.1, 0.1));
        }

        #[test]
        fn lighting_with_pattern_applied() {
            let mut m = Material::default();
            m.pattern = Some(Box::new(Stripe::new(Color::white(), Color::black())));
            m.ambient = 1.0;
            m.diffuse = 0.0;
            m.specular = 0.0;

            let eyev = Vector::new(0.0, 0.0, -1.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
            let sphere = Sphere::new();

            let c1 = m.lighting(
                &light,
                Point::new(0.9, 0.0, 0.0),
                eyev,
                normalv,
                false,
                &sphere,
            );
            let c2 = m.lighting(
                &light,
                Point::new(1.1, 0.0, 0.0),
                eyev,
                normalv,
                false,
                &sphere,
            );

            assert_eq!(c1, Color::white());
            assert_eq!(c2, Color::black());
        }
    }
}
