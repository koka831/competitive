use std::io;
use std::cmp;


/// N <= 10^5, K <= 100
/// dp[i] = 足場iまでのコストの総和の最小値
/// O(NK) <= 10^7
fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let hn = read::<isize>();

    let mut dp = vec![::std::isize::MAX; n];
    dp[0] = 0;

    for i in 1..n {
        for j in 1..cmp::min(k, i) + 1 {
            dp[i] = cmp::min(
                dp[i],
                dp[i - j] + (hn[i] - hn[i - j]).abs(),
            );
        }
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
