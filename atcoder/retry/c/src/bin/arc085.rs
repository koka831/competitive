use std::io;


/// https://beta.atcoder.jp/contests/arc085/tasks/arc085_a
/// t = 1900 * m + 100 * (n - m)
fn main() {
    let (n, m) = {
        let i = read::<u32>();
        (i[0], i[1])
    };
    println!("{}", (1900 * m + 100 * (n - m)) * 2u32.pow(m));
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
