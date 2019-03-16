use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/abc099/tasks/abc099_c
/// Strange Bank
/// 6^p * 6 == 6^(p + 1)
fn main() {
    let n = read_one::<usize>();
    let mut ans = ::std::usize::MAX;
    for i in 0..n + 1 {
        let mut cnt = 0;
        let mut t = i;
        while t > 0 { cnt += t % 6; t /= 6; }
        t = n - i;
        while t > 0 { cnt += t % 9; t /= 9; }
        ans = cmp::min(ans, cnt);
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
