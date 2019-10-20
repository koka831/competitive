use std::io;
use std::cmp;

const INF: usize = 10_000_000_000usize;

/// 補給がある場合, 最短経路長と最適経路が一致しないので,
/// 最短経路長 → 補充しない区間をまとめる → 最短経路長
fn main() {
    let (n, m, l) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut dist = vec![vec![0; n]; n];
    for _ in 0..m {
        let (a, b, c) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1, i[2])
        };
        dist[a][b] = c;
        dist[b][a] = c;
    }

    let mut dp = vec![vec![INF; n]; n];
    for i in 0..n { for j in 0..n {
        if dist[i][j] != 0 { dp[i][j] = dist[i][j]; }
    }}

    for k in 0..n { for i in 0..n { for j in 0..n {
        dp[i][j] = cmp::min(dp[i][j], dp[i][k] + dp[k][j]);
    }}}

    for i in 0..n { for j in 0..n {
        if dp[i][j] <= l { dp[i][j] = 1; }
        else { dp[i][j] = INF; }
    }}

    for k in 0..n { for i in 0..n { for j in 0..n {
        dp[i][j] = cmp::min(dp[i][j], dp[i][k] + dp[k][j]);
    }}}

    let q = read_one::<usize>();
    for _ in 0..q {
        let (s, t) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        if dp[s][t] == INF { println!("-1"); }
        else { println!("{}", dp[s][t] - 1); }
    }
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
