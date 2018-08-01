use std::io;


/// https://beta.atcoder.jp/contests/arc060/tasks/arc060_a
/// (枚数) * A == (選んだカードの和) となるようなknapsack
fn main() {
    let (n, a) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let x = read::<usize>();
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
