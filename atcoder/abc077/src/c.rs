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
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Less => {
                    low = mid + 1;
                },
                _ => { high = mid; }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Greater => {
                    high = mid;
                },
                _ => { low = mid + 1; }
            }
        }
        low
    }
}

fn main() {
    // let n = read_one::<usize>();


    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("{}", a.lower_bound(&2));
    println!("{}", a.upper_bound(&2));
}
