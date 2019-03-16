use std::io;


/// https://beta.atcoder.jp/contests/abc101/tasks/abc101_b
/// https://beta.atcoder.jp/contests/abc080/tasks/abc080_b と同じ
fn main() {
    let n = read_one::<usize>();
    let mut x = n;
    let mut sum = 0;
    while x > 0 {
        sum += x % 10;
        x = x / 10;
    }
    if n % sum == 0 { println!("Yes"); }
    else { println!("No"); }
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
