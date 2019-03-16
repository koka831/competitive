use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/abc070/tasks/abc070_b
fn main() {
    let (a, b, c, d) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3])
    };
    println!("{}", cmp::max(0, cmp::min(b, d) - cmp::max(a, c)));
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
