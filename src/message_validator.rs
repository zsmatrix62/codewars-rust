//! https://www.codewars.com/kata/5fc7d2d2682ff3000e1a3fbc/train/rust

// my solution
fn is_valid_message(msg: &str) -> bool {
    let mut num_vec: Vec<String> = vec![];
    let mut word_vec: Vec<String> = vec![];

    fn is_valid(nv: &Vec<String>, wv: &Vec<String>) -> bool {
        match nv
            .iter()
            .map(|cc| cc.as_ref())
            .collect::<Vec<&str>>()
            .join::<&str>("")
            .parse::<i32>()
        {
            Ok(num) => wv.len() as i32 == num,
            Err(_) => true,
        }
    }

    for c in msg.chars() {
        if c.is_numeric() {
            if !num_vec.is_empty() && !word_vec.is_empty() {
                if !is_valid(&num_vec, &word_vec) {
                    return false;
                } else {
                    num_vec = vec![];
                    word_vec = vec![];
                }
            }
            num_vec.push(c.to_string());
        } else {
            if num_vec.is_empty() {
                return false;
            } else {
                word_vec.push(c.to_string());
            }
        }
    }
    is_valid(&num_vec, &word_vec)
}

// best solution -> without using regex
fn is_valid_message_(msg: &str) -> bool {
    let mut num = 0u32;
    let mut cnt = 0u32;
    for ch in msg.chars() {
        if let Some(mut n) = ch.to_digit(10) {
            if cnt == 0 {
                n = num * 10 + n;
            } else if cnt != num {
                return false;
            }
            num = n;
            cnt = 0;
        } else {
            cnt += 1
        }
    }
    cnt == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(is_valid_message("3hey5hello2hi"), true);
        assert_eq!(is_valid_message("4code13hellocodewars"), true);
        assert_eq!(is_valid_message("3hey5hello2hi5"), false);
        assert_eq!(is_valid_message("code4hello5"), false);
        assert_eq!(is_valid_message("1a2bb3ccc4dddd5eeeee"), true);
        assert_eq!(is_valid_message(""), true);
    }
}
