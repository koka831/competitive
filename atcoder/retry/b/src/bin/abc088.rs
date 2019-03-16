use std::io;


/// https://beta.atcoder.jp/contests/abc088/tasks/abc088_b
fn main() {
    let _n = read_one::<usize>();
    let mut an = read::<isize>();
    an.sort();
    an.reverse();
    let mut alice = 0;
    let mut bob = 0;
    for i in 0..an.len() {
        if i % 2 == 0 { alice += an[i]; }
        else { bob += an[i]; }
    }
    println!("{}", alice - bob);
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
