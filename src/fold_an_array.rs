use std::borrow::BorrowMut;
use std::iter::Zip;

/// https://www.codewars.com/kata/57ea70aa5500adfe8a000110/train/rust

fn fold_array(arr: &[i32], runs: usize) -> Vec<i32> {
    let mut v = arr.to_vec();
    for _ in 0..runs {
        let n = v.len();
        let m = if n & 1 == 1 {
            // is odd
            n / 2
        } else {
            // is even
            1 + n / 2
        };
        for i in 0..m {
            v[i] += v[n - 1 - i]; // add up last m to top m elements
        }
        v.resize(if n & 1 == 1 { 1 + n / 2 } else { n / 2 }, 0);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwiseandtest() {
        assert_eq!(0 & 1, 0);
        assert_eq!(1 & 1, 1);
        assert_eq!(2 & 1, 0);
        assert_eq!(3 & 1, 1);
        assert_eq!(4 & 1, 0);
    }

    #[test]
    fn basic() {
        let input = [1, 2, 3, 4, 5];
        assert_eq!(fold_array(&input, 1), [6, 6, 3]);
        assert_eq!(fold_array(&input, 2), [9, 6]);
        assert_eq!(fold_array(&input, 3), [15]);

        let input = [-9, 9, -8, 8, 66, 23];
        assert_eq!(fold_array(&input, 1), [14, 75, 0]);
    }
}