use std::io;


/// https://beta.atcoder.jp/contests/agc015/tasks/agc015_a
fn main() {
    let (n, a, b) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let ans = if a > b { 0 }
    else if n == 1 {
        if a == b { 1 }
        else { 0 }
    } else {
        (b - a) * (n - 2) + 1
    };

    println!("{}", ans);
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
