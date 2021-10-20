//! https://www.codewars.com/kata/550cc572b9e7b563be00054f/train/rust

fn sjf(jobs: &[usize], index: usize) -> usize {
    let interested = jobs[index];
    jobs.iter().enumerate()
        .filter(|&(idx, job_ccc)| (*job_ccc < interested)
            || (*job_ccc == interested && idx <= index))
        .map(|(_, d)| d)
        .sum()
}

// Rust test example:
//  See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(sjf(&[100], 0), 100);
    assert_eq!(sjf(&[3, 10, 20, 1, 2], 0), 6);
}