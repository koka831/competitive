use std::io;


/// https://beta.atcoder.jp/contests/arc002/tasks/arc002_1
fn main() {
    let y = read_one::<usize>();
    match (y % 4, y % 100, y % 400) {
        (_, _, 0) => println!("YES"),
        (_, 0, _) => println!("NO"),
        (0, _, _) => println!("YES"),
        _ => println!("NO"),
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
