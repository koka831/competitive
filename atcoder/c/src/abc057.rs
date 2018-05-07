use std::io;
use std::cmp;


/// 1 < n < 10^10
/// a * b = n
/// min(max(digits of a, digits of b))
fn main() {
    let n = read_one::<usize>();
    let ans = (1..).take_while(|&x| x * x <= n)
        .filter(|&x| n % x == 0)
        .map(|x| cmp::max(digits(x), digits(n / x)))
        .min()
        .unwrap();

    println!("{}", ans);
}

fn digits(x: usize) -> usize {
    x.to_string().len()
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
