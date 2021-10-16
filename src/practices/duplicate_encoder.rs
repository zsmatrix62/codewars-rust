use std::str::FromStr;

/*
https://www.codewars.com/kata/54b42f9314d9229fd6000d9c/train/rust

The goal of this exercise is to convert a string to a new string
where each character in the new string is
"(" if that character appears only once in the original string, or ")"
if that character appears more than once in the original string.
Ignore capitalization when determining if a character is a duplicate.
 */


fn duplicate_encode(word: &str) -> String {
    let mut res_string = String::new();
    for c in word.chars() {
        if word.chars().filter(|ic| {
            ic.to_uppercase().to_string() == (&c).to_uppercase().to_string()
        }).count() > 1 {
            res_string.push(char::from_str(")").unwrap())
        } else {
            res_string.push(char::from_str("(").unwrap())
        }
    }

    res_string
}


#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}