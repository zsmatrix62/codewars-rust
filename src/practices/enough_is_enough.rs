/// https://www.codewars.com/kata/554ca54ffa7d91b236000023/rust
///
/// Alice and Bob were on a holiday. Both of them took many pictures of the places they've been, and now they want to show Charlie their entire collection. However, Charlie doesn't like these sessions, since the motive usually repeats. He isn't fond of seeing the Eiffel tower 40 times. He tells them that he will only sit during the session if they show the same motive at most N times. Luckily, Alice and Bob are able to encode the motive as a number. Can you help them to remove numbers such that their list contains each number only up to N times, without changing the order?
/// #
/// # Task
///
/// Given a list lst and a number N, create a new list that contains each number of lst at most N times without reordering. For example if N = 2, and the input is [1,2,3,1,2,1,2,3], you take [1,2,3,1,2], drop the next [1,2] since this would lead to 1 and 2 being in the result 3 times, and then take 3, which leads to [1,2,3,1,2,3].
///
/// #
/// # Example
/// ```rust
/// delete_nth(&[20,37,20,21], 1);       // returns vec![20,37,21]
/// delete_nth(&[1,1,3,3,7,2,2,2,2], 3); // returns vec![1, 1, 3, 3, 7, 2, 2, 2])
/// ```

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