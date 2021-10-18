/// https://www.codewars.com/kata/5726f813c8dcebf5ed000a6b/train/rust

fn prime_factor_count(n: i32) -> i32 {
    let (mut n, mut candidate, mut count) = (n, 2, 0);
    while n > 1 {
        while n % candidate == 0 {
            count += 1;
            n /= candidate;
        }
        candidate += 1;
    }
    count
}

fn count_kprimes(k: i32, start: i32, nd: i32) -> Vec<i32> {
    // your code
    (start..=nd).filter(|&n| prime_factor_count(n) == k).collect()
}

fn puzzle(s: i32) -> i32 {
    // your code
    let v_1 = count_kprimes(1, 2, s);
    let v_3 = count_kprimes(3, 8, s);
    let v_7 = count_kprimes(7, 128, s);

    v_7.into_iter()
        .map(|c| {
            v_3.iter()
                .filter(|&b| v_1.iter().any(|&a| a + b + c == s))
                .count()
        })
        .sum::<usize>() as i32
}

fn testing_count_kprimes(k: i32, start: i32, nd: i32, exp: Vec<i32>) -> () {
    assert_eq!(count_kprimes(k, start, nd), exp)
}
#[test]
fn basics_count_kprimes() {
    testing_count_kprimes(5, 1000, 1100, vec![1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100]);
    testing_count_kprimes(12, 100000, 100100, vec![]);
}

fn testing(n: i32, exp: i32) -> () {
    assert_eq!(puzzle(n), exp)
}
#[test]
fn basics_puzzle() {
    testing(100, 0);
    testing(144, 0);
    testing(143, 2);
}