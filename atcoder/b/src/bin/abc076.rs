use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/abc076/tasks/abc076_b
fn main() {
    let n = read_one::<usize>();
    let k = read_one::<usize>();
    let mut ans = 1;
    for _ in 0..n {
        ans = cmp::min(ans * 2, ans + k);
    }
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
