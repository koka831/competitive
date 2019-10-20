use std::io;
use std::cmp;


fn main() {
    let n = read_one::<f64>();
    let mut w = read_one::<usize>();
    w = cmp::min(w, read_one::<usize>());
    w = cmp::min(w, read_one::<usize>());
    w = cmp::min(w, read_one::<usize>());
    w = cmp::min(w, read_one::<usize>());
    println!("{}", 4 + (n / w as f64).ceil() as usize);
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
