//! https://www.codewars.com/kata/5898761a9c700939ee000011/train/rust

fn corner_circle(r: f64) -> f64 {
    let R = (2f64).sqrt() - 1f64;
    (r * R * R * 100f64).round() / 100f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(corner_circle(3.0), 0.51);
        assert_eq!(corner_circle(17.0), 2.92);
    }
}
