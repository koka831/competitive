use std::io;


/// https://beta.atcoder.jp/contests/bitflyer2018-final-open/tasks/bitflyer2018_final_a
fn main() {
    let n = read_one::<usize>();
    let mut pn = Vec::new();
    for _ in 0..n { pn.push(read_one::<usize>()); }
    let ans = pn.iter().map(|i| i.trailing_zeros()).min().unwrap();
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
