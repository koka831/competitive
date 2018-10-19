use std::io;


/// https://beta.atcoder.jp/contests/abc104/tasks/abc104_c
/// All Green
fn main() {
    let (d, g) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut pc = Vec::new();
    for _ in 0..d {
        let i = read::<usize>();
        pc.push((i[0], i[1]));
    }

    for i in 0..(1 << d) {
        for j in 0..d {
            if (i & 1 << j) != 0 { }
        }
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
