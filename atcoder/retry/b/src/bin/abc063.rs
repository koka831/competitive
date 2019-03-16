use std::io;


/// https://beta.atcoder.jp/contests/abc063/tasks/abc063_b
fn main() {
    let mut s = read_one::<String>().chars().collect::<Vec<char>>();
    let bef = s.len();
    s.sort();
    s.dedup();
    let aft = s.len();
    if bef == aft { println!("yes"); }
    else { println!("no"); }
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
