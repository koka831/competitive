use std::io;


fn main() {
    let (n, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vs = Vec::new();
    let mut ws = Vec::new();
    for _ in 0..n {
        let i = read::<usize>();
        vs.push(i[0]);
        ws.push(i[1]);
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in (0..n).rev() { for j in 0..(w + 1) {
        if j < ws[i] { dp[i][j] = dp[i + 1][j]; }
        else {
            dp[i][j] = ::std::cmp::max(dp[i + 1][j], dp[i + 1][j - ws[i]] + vs[i]);
        }
    }}
    println!("{}", dp[0][w]);
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
