//！ https://www.codewars.com/kata/56eb16655250549e4b0013f4

use std::cmp::max;
use std::collections::HashMap;

use chrono::*;

fn most_frequent_days_(year: i32) -> Vec<String> {
    let target_dt = Utc.ymd(year + 1, 1, 1);
    let mut cur_dt = Utc.ymd(year, 1, 1);

    let mut res_map: HashMap<u32, i32> = HashMap::new();
    while cur_dt != target_dt {
        let mut entry = res_map.entry(
            cur_dt.weekday().num_days_from_monday()
        ).or_insert(0);
        *entry += 1;
        cur_dt = cur_dt + Duration::days(1);
    };
    let max = res_map.iter().map(|(_, count)| {
        count
    }).max().unwrap();

    let mut res: Vec<u32> = res_map
        .iter()
        .filter(|&(_, count)| { count == max })
        .map(|(&day, count)| { day })
        .collect();
    res.sort();

    let wk: HashMap<u32, String> = HashMap::from([
        (0, "Monday".into()),
        (1, "Tuesday".into()),
        (2, "Wednesday".into()),
        (3, "Thursday".into()),
        (4, "Friday".into()),
        (5, "Saturday".into()),
        (6, "Sunday".into()),
    ]);
    res.iter().map(|d| {
        wk.get(d).unwrap().into()
    }).collect()
}

// best practice
const WEEKDAY_STR: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

fn most_frequent_days(year: i32) -> Vec<String> {
    let day1 = Utc.ymd(year, 1, 1).weekday().num_days_from_monday();
    let day2 = Utc.ymd(year, 12, 31).weekday().num_days_from_monday();
    let mut vec = vec![day1, day2];
    vec.sort();
    vec.dedup();
    vec.iter().map(|i| WEEKDAY_STR[(*i) as usize].to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(most_frequent_days(1770), ["Monday"]);
        assert_eq!(most_frequent_days(1785), ["Saturday"]);
        assert_eq!(most_frequent_days(1794), ["Wednesday"]);
        assert_eq!(most_frequent_days(1901), ["Tuesday"]);
        assert_eq!(most_frequent_days(1910), ["Saturday"]);
        assert_eq!(most_frequent_days(1968), ["Monday", "Tuesday"]);
        assert_eq!(most_frequent_days(1984), ["Monday", "Sunday"]);
        assert_eq!(most_frequent_days(1986), ["Wednesday"]);
        assert_eq!(most_frequent_days(2001), ["Monday"]);
        assert_eq!(most_frequent_days(2016), ["Friday", "Saturday"]);
        assert_eq!(most_frequent_days(2135), ["Saturday"]);
        assert_eq!(most_frequent_days(3043), ["Sunday"]);
        assert_eq!(most_frequent_days(3150), ["Sunday"]);
        assert_eq!(most_frequent_days(3230), ["Tuesday"]);
        assert_eq!(most_frequent_days(3361), ["Thursday"]);
        assert_eq!(most_frequent_days(2000), ["Saturday", "Sunday"]);
        assert_eq!(most_frequent_days(1984), ["Monday", "Sunday"]);
    }
}

