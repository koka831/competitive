use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/agc001/tasks/agc001_a
fn main() {
    let _ = read_one::<usize>();
    let mut ln = read::<isize>();
    ln.sort();
    let mut ans = 0;
    for i in 0..ln.len() {
        if i % 2 != 0 { continue; }
        ans += cmp::min(ln[i], ln[i + 1]);
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
