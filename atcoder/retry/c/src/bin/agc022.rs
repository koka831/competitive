use std::io;


/// https://beta.atcoder.jp/contests/agc022/tasks/agc022_a
fn main() {
    let  s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut s_clone = s.clone();
    s_clone.sort();

    println!("{:?}", s);
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
