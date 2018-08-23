use std::io;


/// https://beta.atcoder.jp/contests/abc064/tasks/abc064_b
fn main() {
    let _ = read_one::<usize>();
    let mut an = read::<isize>();
    an.sort();
    let mut ans = 0;
    let mut prev = an.pop().unwrap();
    for a in an.iter().rev() {
        ans += (a - prev).abs();
        prev = *a;
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
