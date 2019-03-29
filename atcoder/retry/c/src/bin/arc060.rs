use std::io;


fn main() {
    let (n, a) = {
        let i = read::<usize>();
        (i[0], i[1] - 1)
    };

    let xn = read::<usize>();
    let mut yn = Vec::new();
    for x in xn { yn.push(x as isize - a as isize); }
    let max = yn.iter().max().unwrap() as usize;
    let mut dp = vec![vec![0; n * max * 2 + 1]; n];

    for i in 0..n { for j in 0..n * max * 2 + 1 {
        let ty = j - yn[i];
        dp[i][j] = if i == 0 && j == n * max { 1 }
        else if i > 0 && (ty < 0 || ty > (2 * n * max) as isize) { dp[i - 1][j] }
        else if i > 0 && 0 <= (j as isize ) { dp[i - 1][j] + dp[i - 1][ty as usize] }
        else { 0 }
    }}

    println!("{}", dp[n - 1][n * max - 1] - 1);
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
