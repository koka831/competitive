use std::io;


/// https://beta.atcoder.jp/contests/abc084/tasks/abc084_c
fn main() {
    let n = read_one::<usize>();
    let mut vec = Vec::new();
    for _ in 1..n {
        let i = read::<usize>();
        vec.push((i[0], i[1], i[2]));
    }

    println!("{:?}", vec);
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
