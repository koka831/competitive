use std::io;
use std::cmp;


fn main() {
    let n = read_one::<isize>();
    let mut ans = 100_000_000_000;

    if n == 1 { println!("0"); return; }
    for i in 1..n { for j in 1..(n / i) + 1 {
        if i * j > n { continue; }
        ans = cmp::min(ans, (i - j).abs() + n - i * j);
    }}

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
