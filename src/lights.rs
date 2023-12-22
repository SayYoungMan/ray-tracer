use crate::{color::Color, tuples::SpatialTuple};

pub struct PointLight {
    pub position: SpatialTuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: SpatialTuple, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::new_point;

    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color(1.0, 1.0, 1.0);
        let position = new_point(0.0, 0.0, 0.0);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
