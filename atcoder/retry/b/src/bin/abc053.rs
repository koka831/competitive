use std::io;


/// https://beta.atcoder.jp/contests/abc053/tasks/abc053_b
fn main() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut idx_a = 0;
    let mut idx_z = 0;

    for j in 0..s.len() {
        if s[j] == 'A' { idx_a = j; break; }
    }

    for j in (0..s.len()).rev() {
        if s[j] == 'Z' { idx_z = j; break; }
    }

    println!("{}", idx_z - idx_a + 1);
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
