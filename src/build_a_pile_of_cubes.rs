//! https://www.codewars.com/kata/5592e3bd57b64d00f3000047

fn find_nb(n: u64) -> i32 {
    let mut sum = 0_u64;
    let l = (0u64..)
        .take_while(|&x| {
            sum += x.pow(3);
            sum < n
        })
        .count() as i32;
    if sum == n {
        l
    } else {
        -1
    }
}

fn testing(n: u64, exp: i32) -> () {
    assert_eq!(find_nb(n), exp);
}

#[test]
fn basics_find_nb() {
    testing(4183059834009, 2022);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}
