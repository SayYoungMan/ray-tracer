use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    color_grid: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut color_matrix = Vec::with_capacity(height);

        for _ in 0..height {
            let row = vec![Color(0.0, 0.0, 0.0); width];
            color_matrix.push(row);
        }

        Canvas {
            width,
            height,
            color_grid: color_matrix,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.color_grid[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.color_grid[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.pixel_at(0, 0), Color(0.0, 0.0, 0.0));
        assert_eq!(c.pixel_at(9, 19), Color(0.0, 0.0, 0.0));
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }
}
