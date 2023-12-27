use crate::constants::EPSILON;

pub fn zero_if_trivial(n: f64) -> f64 {
    if n.abs() < EPSILON {
        0.0
    } else {
        n
    }
}
