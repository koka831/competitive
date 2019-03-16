use std::io;
// https://qiita.com/drken/items/97e37dd6143e33a64c8c
// binary_search

fn main() {}

fn binary_search<T: Ord>(a: &[T], key: usize) -> usize {
    let mut left: isize = -1;
    let mut right: isize = a.len() as isize;

    while (right - left).abs() > 1 {
        let mid = left + (right - left) / 2;

        if cond(mid, key as isize) { right = mid; }
        else { left = mid; }
    }

    right as usize
}

fn cond<T: Ord>(idx: T, key: T) -> bool {
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
