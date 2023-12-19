use std::io::Write;
use std::{fs::File, io};

use crate::{color::Color, constants::MAX_COLOR_VALUE};

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

    pub fn with_filled_color(width: usize, height: usize, color: Color) -> Canvas {
        let mut color_matrix = Vec::with_capacity(height);

        for _ in 0..height {
            let row = vec![color; width];
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

    fn construct_ppm(self) -> String {
        let header = self.construct_ppm_header();
        let body = self.construct_ppm_body();

        header + "\n" + &body
    }

    fn construct_ppm_header(&self) -> String {
        format!("P3\n{} {}\n{}", self.width, self.height, MAX_COLOR_VALUE)
    }

    fn construct_ppm_body(&self) -> String {
        let mut body = String::new();
        let mut last_newline_idx = 0;

        for row in &self.color_grid {
            for (i, color) in row.iter().enumerate() {
                let color_value_string = format!(
                    "{} {} {} ",
                    clamp_and_scale_color_value(color.0),
                    clamp_and_scale_color_value(color.1),
                    clamp_and_scale_color_value(color.2)
                );
                body += &color_value_string;

                // This is to make sure each line in PPM file does not go over 70
                if body.len() - last_newline_idx > 58 && i != row.len() - 1 {
                    body.pop();
                    body += "\n";
                    last_newline_idx = body.len() - 1;
                }
            }
            body.pop();
            body += "\n";
            last_newline_idx = body.len() - 1;
        }

        body
    }
    pub fn to_ppm(self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;

        write!(file, "{}", self.construct_ppm())?;

        Ok(())
    }
}

fn clamp_and_scale_color_value(c: f64) -> u8 {
    if c < 0.0 {
        return 0;
    }
    if c > 1.0 {
        return MAX_COLOR_VALUE;
    }
    return (c * f64::from(MAX_COLOR_VALUE)).round() as u8;
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

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.construct_ppm_header();
        let expected_literal = "P3\n5 3\n255";

        assert_eq!(ppm, String::from(expected_literal));
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);

        let c1 = Color(1.5, 0.0, 0.0);
        let c2 = Color(0.0, 0.5, 0.0);
        let c3 = Color(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.construct_ppm_body();
        let expected_literal = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";

        assert_eq!(ppm, String::from(expected_literal));
    }

    #[test]
    fn splitting_long_lines() {
        let background_color = Color(1.0, 0.8, 0.6);
        let c = Canvas::with_filled_color(10, 2, background_color);

        let ppm = c.construct_ppm_body();
        let expected_literal = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n";

        assert_eq!(ppm, String::from(expected_literal));
    }
}
