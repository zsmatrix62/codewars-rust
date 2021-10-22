//！ https://www.codewars.com/kata/59aa6567485a4d03ff0000ca/train/rust

fn solve(a: u32, b: u32) -> usize {
    (a..b)
        .filter(|&num| {
            if num <= 1 {
                return false;
            }
            let w = (num as f32).sqrt() as u32;
            for i in 2..=w {
                if num % i == 0 {
                    return false;
                }
            }
            true
        })
        .filter(|&w| {
            let mut s = w.to_string();
            loop {
                let ss: i32 = s
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<i32>())
                    .filter(|r| r.is_ok())
                    .map(|w| {
                        let n = w.unwrap();
                        n.pow(2)
                    })
                    .sum();

                if ss == 1 || ss == 7 {
                    return true;
                } else if ss < 10 {
                    return false;
                }
                s = ss.to_string()
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_tests() {
        assert_eq!(solve(1, 25), 4);
        assert_eq!(solve(100, 1000), 28);
        assert_eq!(solve(100, 2000), 47);
        assert_eq!(solve(100, 3000), 65);
        assert_eq!(solve(100, 4000), 95);
    }
}
