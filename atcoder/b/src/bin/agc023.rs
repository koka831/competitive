use std::io;
use std::collections::HashMap;


/// https://beta.atcoder.jp/contests/agc023/tasks/agc023_a
fn main() {
    let n = read_one::<usize>();
    let an = read::<isize>();
    let mut sn = vec![0; n + 1];
    for i in 0..n {
        sn[i + 1] = sn[i] + an[i];
    }

    let mut hm = HashMap::new();
    for s in &sn {
        *hm.entry(s).or_insert(0) += 1;
    }

    let ans = hm.values()
        .filter(|n| **n > 1)
        .map(|&n| ncr(n as usize, 2)).sum::<usize>();

    println!("{}", ans);
}

fn ncr(n: usize, r: usize) -> usize {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => ncr(n, r - 1) * (n - r + 1) / r,
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
