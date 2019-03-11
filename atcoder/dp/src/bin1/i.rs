use std::io;


fn main() {
    let n = read_one::<usize>();
    let pn = read::<f64>();

    let mut dp = vec![vec![0f64; n + 1]; n];
    dp[0][0] = 1f64 - pn[0];
    dp[0][1] = pn[0];

    for i in 1..n { for j in 0..i + 1 {
        if j == 0 {
            dp[i][j] = dp[i - 1][j] * (1f64 - pn[i]);
        } else {
            dp[i][j] = dp[i - 1][j - 1] * pn[i] + dp[i - 1][j] * (1f64 - pn[i]);
        }
    }}

    let mut sum = 0f64;
    for i in (n / 2)..n {
        println!("{}", i);
        sum += dp[n - 1][i];
    }

    println!("{:?}", dp);
    println!("{}", sum);
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
