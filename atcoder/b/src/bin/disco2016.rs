use std::io;


/// https://beta.atcoder.jp/contests/ddcc2016-qual/tasks/ddcc_2016_qual_b
fn main() {
    let (r, n, m) = {
        let i = read::<usize>();
        (i[0] as f64, i[1], i[2])
    };
    let d = (2f64 * r) / n as f64;
    let mut cost = vec![0f64; n + 2 * m];
    for i in m..n + m {
        cost[i] = ((r - (d * (i - m) as f64)) / r).asin().cos() * 2f64 * r;
    }
    let mut ans = 0f64;
    for i in 0..n + m {
        ans += if cost[i] > cost[i + m] { cost[i] } else { cost[i + m] };
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
