/// https://www.codewars.com/kata/554ca54ffa7d91b236000023/rust

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    lst.iter().for_each(|&i| if result.iter().filter(|&x| x == &i).count() < n {
        result.push(i);
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
        assert_eq!(delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }
}