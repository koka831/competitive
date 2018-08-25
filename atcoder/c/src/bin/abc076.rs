use std::io;


/// https://beta.atcoder.jp/contests/abc076/tasks/abc076_c
fn main() {
    let s = read_one::<String>()
        .chars().collect::<Vec<char>>();
    let t = read_one::<String>()
        .chars().collect::<Vec<char>>();

    if t.len() > s.len() { println!("UNRESTORABLE"); return; }

    for i in 0..(s.len() - t.len()) {
        let mut buf = s.clone();
        for j in 0..t.len() {
            if buf[i + j] == '?' { buf[i + j] = t[j]; }
        }
    }
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
