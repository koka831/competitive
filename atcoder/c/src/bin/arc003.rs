use std::io;


/// https://beta.atcoder.jp/contests/arc003/tasks/arc003_1
fn main() {
    let n = read_one::<usize>();
    let score = read_one::<String>()
        .chars()
        .map(|c| match c {
            'A' => 4.0,
            'B' => 3.0,
            'C' => 2.0,
            'D' => 1.0,
            _   => 0.0,
        }).sum::<f64>() / n as f64;
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
