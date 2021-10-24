//! https://www.codewars.com/kata/56dbf59b0a10feb08c000227/train/rust

use std::collections::VecDeque;
use std::str::FromStr;

fn pros(s: &str, reverse: bool) -> String {
    let mut blocks = s.split("\n").map(|a| {
        a.chars().rev().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();
    if reverse {
        blocks.reverse();
    }
    let mut res: String = String::new();

    let len = blocks.first().unwrap().len();
    (0..len).for_each(|idx| {
        blocks.iter().for_each(|cv| {
            res.push(cv[idx]);
        });
        res.push_str("\n");
    });

    res.trim().into()
}

fn diag_2_sym(s: &str) -> String {
    pros(s, true)
}

fn rot_90_counter(s: &str) -> String {
    pros(s, false)
}

fn selfie_diag2_counterclock(s: &str) -> String {
    let s1 = diag_2_sym(s);
    let s2 = rot_90_counter(s);

    s.split("\n").zip(s1.split("\n"))
        .zip(s2.split("\n")).map(|d| {
        format!("{}\n", vec![d.0.0, d.0.1, d.1].join("|"))
    }).collect::<String>().trim().into()
}

fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing1(s: &str, exp: &str) -> () {
        assert_eq!(oper(diag_2_sym, s), exp.to_string())
    }

    fn testing2(s: &str, exp: &str) -> () {
        assert_eq!(oper(rot_90_counter, s), exp.to_string())
    }

    fn testing3(s: &str, exp: &str) -> () {
        assert_eq!(oper(selfie_diag2_counterclock, s), exp.to_string())
    }

    #[test]
    fn basics_oper() {
        testing1("LmvLyg\nDKELBm\nylJhui\nXRXqHD\nzlisCT\nhPqxYb",
                 "bTDimg\nYCHuBy\nxsqhLL\nqiXJEv\nPlRlKm\nhzXyDL");

        testing2("EcGcXJ\naaygcA\nNgIshN\nyOrCZE\neBEqpm\nNkxCgw",
                 "JANEmw\nXchZpg\ncgsCqC\nGyIrEx\ncagOBk\nEaNyeN");


        testing3("NJVGhr\nMObsvw\ntPhCtl\nsoEnhi\nrtQRLK\nzjliWg",
                 "NJVGhr|gKilwr|rwliKg\nMObsvw|WLhtvh|hvthLW\ntPhCtl|iRnCsG|GsCnRi\nsoEnhi|lQEhbV|VbhEQl\nrtQRLK|jtoPOJ|JOPotj\nzjliWg|zrstMN|NMtsrz");
    }
}