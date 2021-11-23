// https://www.codewars.com/kata/514b92a657cdc65150000006

fn solution(num: i32) -> i32 {
    (0..num).filter(|xx| {
        xx % 5 == 0 || xx % 3 == 0
    }).sum()
}

#[test]
fn returns_expected() {
    assert_eq!(solution(10), 23);
    assert_eq!(solution(11), 33);
    assert_eq!(solution(6), 8);
    assert_eq!(solution(-3), 0);
}