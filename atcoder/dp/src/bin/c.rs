use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut nn = vec![Vec::new(); 3];

    for _ in 0..n {
        let (a_i, b_i, c_i) = {
            let i = read::<usize>();
            (i[0], i[1], i[2])
        };
        nn[0].push(a_i);
        nn[1].push(b_i);
        nn[2].push(c_i);
    }

    // 0: A
    // 1: B
    // 2: C
    let mut dp = vec![vec![0; n]; 3];
    for i in 0..3 {
        dp[i][0] = nn[i][0];
    }

    for i in 1..n { for j in 0..3 {
        let mut prev = 0;
        for k in 0..3 {
            if k == j { continue; }
            prev = cmp::max(prev, dp[k][i - 1]);
        }
        dp[j][i] = prev + nn[j][i];
    }}

    println!("{}", cmp::max(
        dp[0][n - 1],
        cmp::max(dp[1][n - 1], dp[2][n - 1])
        )
    );
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
