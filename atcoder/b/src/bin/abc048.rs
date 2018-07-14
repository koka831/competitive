use std::io;


/// https://beta.atcoder.jp/contests/abc048/tasks/abc048_b
fn main() {
    let (a, b, x) = {
        let i = read::<isize>();
        (i[0], i[1], i[2])
    };

    println!("{}", count(b, x) - count(a - 1, x));
}

fn count(n: isize, x: isize) -> isize {
    if n >= 0 { n / x + 1 }
    else { 0 }
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
