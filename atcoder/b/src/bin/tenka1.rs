use std::io;


/// https://beta.atcoder.jp/contests/tenka1-2017-beginner/tasks/tenka1_2017_b
fn main() {
    let n = read_one::<usize>();
    let mut ab = Vec::new();
    for _ in 0..n {
        let i = read::<usize>();
        ab.push((i[0], i[1]));
    }
    ab.sort();
    ab.reverse();
    println!("{}", ab[0].0 + ab[0].1);
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
