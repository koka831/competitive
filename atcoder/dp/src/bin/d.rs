use std::io;
use std::cmp;


fn main() {
    let (n, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut wn = Vec::new();
    let mut vn = Vec::new();

    for _ in 0..n {
        let (w_i, v_i) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        wn.push(w_i);
        vn.push(v_i);
    }

    let mut dp = vec![vec![0usize; w + 1]; n + 1];

    for i in (0..n).rev() { for j in 0..w + 1 {
        if j < wn[i] {
            dp[i][j] = dp[i + 1][j];
        } else {
            dp[i][j] = cmp::max(
                dp[i + 1][j],
                dp[i + 1][j - wn[i]] + vn[i]
            );
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
