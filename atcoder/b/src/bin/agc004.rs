use std::io;


/// https://beta.atcoder.jp/contests/agc004/tasks/agc004_a
fn main() {
    let mut abc = read::<usize>();

    if (abc[0] * abc[1] * abc[2]) % 2 == 0 { println!("0"); }
    else {
        abc.sort();
        println!("{}", abc[0] * abc[1]);
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
