use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/abc075/tasks/abc075_b
fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut s = Vec::new();
    for _ in 0..h {
        let i = read_one::<String>().chars().collect::<Vec<char>>();
        s.push(i);
    }

    for i in 0..h { for j in 0..w {
    }}
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
