use std::io;
use std::cmp;
use std::collections::HashMap;


fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut hm = HashMap::new();
    for _ in 0..h {
        let s = read_one::<String>().chars().collect::<Vec<_>>();
        for c in s { *hm.entry(c).or_insert(0) += 1; }
    }

    let mut cnt = Vec::new();
    for c in hm.values() { cnt.push(c); }
    let ans = if h == 1 || w == 1 || h * w % 2 != 0 {
        validate_odd(&cnt) && validate_even(&cnt, cmp::ma)
    }
}

fn validate_odd(ns: &[usize]) -> bool {
    true
}

fn validate_even(ns: &[usize], level: usize) -> bool {
    true
}


#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
std::str::FromStr,
T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
std::str::FromStr,
T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
