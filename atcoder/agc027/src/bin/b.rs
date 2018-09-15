use std::io;
use std::cmp;


fn main() {
    let (n, x) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let xn = read::<usize>();
    let mut dp = vec![vec![100_000_000_000; n + 1]; n + 1];
    for i in 0..n {
        dp[0][i] = 2 * x + (i + 2) * (i + 2) * xn[0];
    }
    for i in 1..n { for j in 0..n {
        dp[i][j] = x + cmp::min(
            (j + 2) * (j + 2) * xn[i] + x + xn[i - 1] + dp[i - 1][0],
            (j + 2) * (j + 2) * (xn[i] - xn[i - 1]) + dp[i - 1][j + 1]
        );
    }}

    println!("{}", dp[n - 1][0] + xn[n - 1]);
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
