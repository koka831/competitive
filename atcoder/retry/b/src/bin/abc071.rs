use std::io;


/// https://beta.atcoder.jp/contests/abc071/tasks/abc071_b
fn main() {
    let s = read_one::<String>();
    let az = "abcdefghijklmnopqrstuvwxyz".to_string()
        .chars().collect::<Vec<char>>();

    for c in az {
        if !s.contains(&c.to_string()) { println!("{}", c); return; }
    }
    println!("None");
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
