use std::io;


/// https://beta.atcoder.jp/contests/arc063/tasks/arc063_a
fn main() {
    let s = read_one::<String>()
        .chars().collect::<Vec<char>>();
    let mut cnt = 0;
    let mut c = s[0];
    for i in 1..s.len() {
        if c != s[i] { cnt += 1; c = s[i]; }
    }

    println!("{}", cnt);
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
