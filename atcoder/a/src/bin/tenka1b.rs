use std::io;


/// https://beta.atcoder.jp/contests/tenka1-2016-qualb/tasks/tenka1_2016_qualB_a
fn main() {
    println!("{}", f(f(f(20f64) as f64) as f64));
}

fn f(n: f64) -> usize {
    ((n*n + 4f64) / 8f64).floor() as usize
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
