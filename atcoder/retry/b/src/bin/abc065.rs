use std::io;


/// https://beta.atcoder.jp/contests/abc065/tasks/abc065_b
fn main() {
    let n = read_one::<usize>();
    let mut an = Vec::new();

    for _ in 0..n {
        let i = read_one::<usize>() - 1;
        an.push(i);
    }
    let mut btn = 0;
    for i in 0..n {
        if btn == 1 { println!("{}", i); return; }
        btn = an[btn];
    }
    println!("-1");
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
