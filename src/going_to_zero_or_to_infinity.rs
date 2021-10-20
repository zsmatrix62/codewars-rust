//!https://www.codewars.com/kata/55a29405bc7d2efaff00007c/train/rust
//! C_k+1 = 1 + C_k / (k + 1);

fn going(n: i32) -> f64 {
    let ans = (2..=n).fold(1.0, |acc, i| 1.0 + acc / (i as f64));
    (ans * 1e6).floor() / 1e6
}

#[cfg(test)]
mod tests {
    use float_eq::float_eq;

    use super::*;

    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-12;
        let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
    }

    fn dotest(n: i32, exp: f64) -> () {
        assert_float_equals(going(n), exp);
    }

    #[test]
    fn basics_going() {
        dotest(5, 1.275);
        dotest(6, 1.2125);
        dotest(7, 1.173214);
        dotest(8, 1.146651);
    }
}