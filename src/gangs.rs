// https://www.codewars.com/kata/60490a215465720017ab58fa


use std::collections::HashSet;

fn gangs(divisors: &[u32], k: u32) -> u32 {
    (1..=k).map(|x| divisors.iter().filter(|&y| x % y == 0).collect::<Vec<_>>())
        .collect::<HashSet<_>>().len() as u32
}


// Rust translation by B1ts 2021-03
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::gangs;

    #[test]
    fn sample_tests() {
        assert_eq!(gangs(&[2, 3], 6), 4, "k = 4, divisors = [2, 3]");
        assert_eq!(gangs(&[2, 3, 6, 5], 15), 7, "k = 7, divisors = [2, 3, 6, 5]");
    }
}
