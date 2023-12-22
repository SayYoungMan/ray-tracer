use crate::color::Color;

#[derive(Debug, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn default() -> Self {
        Self::new(Color(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.color, Color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
