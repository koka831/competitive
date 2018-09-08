use std::io;


/// https://beta.atcoder.jp/contests/arc060/tasks/arc060_a
/// (枚数) * A == (選んだカードの和) となるようなknapsack
fn main() {
    let (n, a) = {
        let i = read::<usize>();
        (i[0], i[1] - 1)
    };

    let x = read::<usize>();
    let x_max = x.iter().max().unwrap();
    let mut dp = vec![vec![vec![0; n * x_max + 1]; n]; n];

    for i in 0..n { for j in 0..n { for k in 0..n * x_max + 1 {
        dp[i][j][k] = if i * j * k == 0 { 0 }
        else if i > 0 && k < x[i] {
            dp[i - 1][j][k]
        } else if j > 0 && j > 0 && k >= x[i] {
            dp[i - 1][j][k] + dp[i - 1][j - 1][k - x[i]]
        } else { 0 };
    }}}

    let mut ans = 0;
    for i in 0..n {
        ans += dp[n - 1][i][i * a];
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
