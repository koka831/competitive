use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/arc083/tasks/arc083_a
fn main() {
    let (a, b, c, d, e, f) = {
        let i = read::<u32>();
        (i[0], i[1], i[2], i[3], i[4], i[5])
    };

    let mut ans = 0;
    for i in 0..31 { for j in 0..31 {

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
