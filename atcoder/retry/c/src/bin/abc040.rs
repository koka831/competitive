use std::io;
use std::cmp;

/// https://beta.atcoder.jp/contests/abc040/tasks/abc040_c
fn main() {
    let n = read_one::<usize>();
    let an = read::<isize>();
    let mut dp = vec![::std::usize::MAX; n];

    dp[0] = 0;
    dp[1] = (an[0] - an[1]).abs() as usize;
    for i in 2..n {
        dp[i] = cmp::min(
            dp[i - 1] + (an[i] - an[i - 1]).abs() as usize,
            dp[i - 2] + (an[i] - an[i - 2]).abs() as usize,
        );
    }

    println!("{}", dp[n - 1]);
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
