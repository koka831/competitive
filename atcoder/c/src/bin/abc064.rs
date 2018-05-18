use std::io;
use std::collections::HashSet;

/// https://beta.atcoder.jp/contests/abc064/tasks/abc064_c
fn main() {
    let _n = read_one::<usize>();
    let vec = read::<usize>();

    let under = vec.iter()
        .filter(|&x| *x < 3200)
        .map(|x| x / 400)
        .collect::<HashSet<usize>>()
        .len();

    let over = vec.iter()
        .filter(|&x| *x >= 3200)
        .count();

    if under == 0 {
        println!("1 {}", over);
    } else {
        println!("{} {}", under, under + over);
    }
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
