use std::io;


/// https://beta.atcoder.jp/contests/abc068/tasks/abc068_b
fn main() {
    let n = read_one::<usize>();
    let mut cnt = 0;
    let mut ans = 0;
    for i in 0..n {
        let zeros = (i + 1).trailing_zeros();
        if cnt <= zeros { ans = i + 1; cnt = zeros; }
    }
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
