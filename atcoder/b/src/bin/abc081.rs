use std::io;


/// https://beta.atcoder.jp/contests/abc081/tasks/abc081_b
fn main() {
    let _ = read_one::<usize>();
    let an = read::<usize>().iter()
        .map(|i| i.trailing_zeros())
        .min().unwrap();
    println!("{}", an);
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
