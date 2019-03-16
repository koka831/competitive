use std::io;
use std::cmp;


/// 2 <= N <= 10^5
/// dp[i]: 足場iまでに支払うコストの総和の最小値
fn main() {
    let n = read_one::<usize>();
    let hn = read::<isize>();

    let mut dp = vec![::std::isize::MAX; n];
    dp[0] = 0;
    dp[1] = (hn[1] - hn[0]).abs();

    for i in 2..n {
        dp[i] = cmp::min(
            (hn[i - 1] - hn[i]).abs() + dp[i - 1],
            (hn[i - 2] - hn[i]).abs() + dp[i - 2],
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
