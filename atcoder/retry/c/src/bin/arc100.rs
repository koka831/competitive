use std::io;


/// https://beta.atcoder.jp/contests/arc100/tasks/arc100_a
fn main() {
    let n = read_one::<usize>();
    let an = read::<isize>();

    let mut bn = Vec::new();
    for i in 0..n {
        bn.push(an[i] - i as isize + 1);
    }
    bn.sort();
    let mut ans = 0;
    let med = bn[bn.len() / 2];
    for b in bn {
        ans += (b - med).abs();
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
