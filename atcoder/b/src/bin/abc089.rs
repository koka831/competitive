use std::io;


/// https://beta.atcoder.jp/contests/abc089/tasks/abc089_b
fn main() {
    let _n = read_one::<usize>();
    let mut sn = read::<String>();
    sn.sort();
    sn.dedup();
    if sn.len() == 3 { println!("Three"); }
    else { println!("Four"); }
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
