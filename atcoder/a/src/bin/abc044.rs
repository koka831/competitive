use std::io;
use std::cmp;


fn main() {
    let n = read_one::<isize>();
    let k = read_one::<isize>();
    let x = read_one::<isize>();
    let y = read_one::<isize>();
    println!("{}", cmp::min(n, k) * x + cmp::max(0, n - k) * y);

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
