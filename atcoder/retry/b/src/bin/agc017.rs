
use std::io;


/// https://beta.atcoder.jp/contests/agc017/tasks/agc017_a
fn main() {
    let (n, p) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();

    let mut flg = true;
    for i in 0..an.len() { if an[i] % 2 != 0 { flg = false; }}
    let ans = if flg {
        if p == 1 { 0 }
        else { 2u64.pow(n as u32) }
    } else { 2u64.pow((n - 1) as u32) };
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
