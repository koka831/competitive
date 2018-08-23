use std::io;
use std::collections::HashMap;


/// https://beta.atcoder.jp/contests/abc061/tasks/abc061_b
fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut hm = HashMap::new();
    for _ in 0..m {
        let i = read::<usize>();
        *hm.entry(i[0] - 1).or_insert(0) += 1;
        *hm.entry(i[1] - 1).or_insert(0) += 1;
    }

    for i in 0..n {
        println!("{}", hm.get(&i).unwrap_or(&0));
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
