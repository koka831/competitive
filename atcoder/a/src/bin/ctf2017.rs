use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/code-thanks-festival-2017-open/tasks/code_thanks_festival_2017_a
fn main() {
    let mut ans = 0;
    for _ in 0..8 {
        ans = cmp::max(ans, read_one::<usize>());
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
