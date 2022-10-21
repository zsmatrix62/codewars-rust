// https://www.codewars.com/kata/52503c77e5b972f21600000e/train/rust
//
// Write a function which takes an array data of numbers and returns the largest difference in indexes j - i such that data[i] <= data[j]

fn largest_difference(data: &[i32]) -> usize {
    for distance in (2..=data.len()).rev() {
        for window in data.windows(distance) {
            if window[0] <= *window.last().unwrap() {
                return distance - 1;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::largest_difference;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(data: &[i32], expected: usize) {
        assert_eq!(largest_difference(data), expected, "{ERR_MSG} with data = {data:?}")
    }

    #[test]
    fn sample_tests() {
        dotest(&[9, 4, 1, 10, 3, 4, 0, -1, -2], 4);
        dotest(&[3, 2, 1], 0);
        dotest(&[1, 2, 3], 2);
        dotest(&[9, 4, 1, 2, 3, 0, -1, -2], 2);
        dotest(&[9, 4, 1, 2, 3, 4, 0, -1, -2], 4);
        dotest(&[78, 88, 64, 94, 17, 91, 57, 69, 38, 62, 13, 17, 35, 15, 20], 10);
        dotest(&[4, 3, 3, 1, 5, 2], 4);
    }
}
