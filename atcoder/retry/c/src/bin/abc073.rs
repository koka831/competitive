use std::io;
use std::collections::HashMap;


/// https://beta.atcoder.jp/contests/abc073/tasks/abc073_c
/// A_i < 10^9
fn main() {
    let n = read_one::<usize>();
    let mut hm = HashMap::new();

    for _ in 0..n {
        let i = hm.entry(read_one::<usize>())
            .or_insert(0);
        *i += 1;
    }

    let ans = hm.values()
        .filter(|&x| x % 2 != 0)
        .count();
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
