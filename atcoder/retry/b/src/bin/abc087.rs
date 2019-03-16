use std::io;


/// https://beta.atcoder.jp/contests/abc087/tasks/abc087_b
fn main() {
    let a = read_one::<usize>() + 1;
    let b = read_one::<usize>() + 1;
    let c = read_one::<usize>() + 1;
    let x = read_one::<usize>();
    let mut cnt = 0;
    for i in 0..a { for j in 0..b { for k in 0..c {
        if i * 500 + j * 100 + k * 50 == x { cnt += 1; }
    }}}
    println!("{}", cnt)
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
