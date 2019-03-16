use std::io;


/// https://beta.atcoder.jp/contests/abc082/tasks/abc082_b
fn main() {
    let mut s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut t = read_one::<String>().chars().collect::<Vec<char>>();
    s.sort();
    t.sort();
    t.reverse();
    if s < t { println!("Yes"); }
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
