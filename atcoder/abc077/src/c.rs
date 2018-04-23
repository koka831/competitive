#![allow(unused_imports)]
use std::io;
use std::cmp;

#[allow(unused)]
static MOD : i64 = (1e9 as i64) + 7;

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

pub trait BinarySearch<T> {

}

impl<T: Ord> BinarySearch<T> for [T] {

}

fn main() {
    let n = read_one::<usize>();

}
