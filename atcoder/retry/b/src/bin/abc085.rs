use std::io;


/// https://beta.atcoder.jp/contests/abc085/tasks/abc085_b
fn main() {
    let n = read_one::<usize>();
    let mut dn = Vec::new();
    for _ in 0..n {
        dn.push(read_one::<usize>());
    }
    dn.sort();
    dn.dedup();
    println!("{}", dn.len());
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
