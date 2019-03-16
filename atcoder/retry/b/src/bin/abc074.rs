use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/abc074/tasks/abc074_b
fn main() {
    let _ = read_one::<usize>();
    let k = read_one::<isize>();
    let x = read::<isize>();
    let ans = x.iter().fold(0, |sum, a| sum + cmp::min(a, &(a - k).abs()) * 2);
    println!("{}", ans);
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
