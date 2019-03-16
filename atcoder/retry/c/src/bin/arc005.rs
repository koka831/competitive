use std::io;


/// https://beta.atcoder.jp/contests/arc005/tasks/arc005_1
fn main() {
    let _ = read_one::<usize>();
    let score = read_one::<String>()
        .replace(".", "")
        .split_whitespace()
        .filter(|&s| {
            s == "Takahashikun" ||
            s == "takahashikun" ||
            s == "TAKAHASHIKUN"
        }).count();
    println!("{}", score);
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
