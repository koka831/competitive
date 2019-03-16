use std::io;


/// https://beta.atcoder.jp/contests/agc003/tasks/agc003_a
fn main() {
    let s = read_one::<String>();
    let mut flg = true;
    if s.contains(&"N") { if !s.contains(&"S") { flg = false; }}
    if s.contains(&"S") { if !s.contains(&"N") { flg = false; }}
    if s.contains(&"W") { if !s.contains(&"E") { flg = false; }}
    if s.contains(&"E") { if !s.contains(&"W") { flg = false; }}
    if flg { println!("Yes"); }
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
