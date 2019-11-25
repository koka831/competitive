use std::io;

fn main() {
    let (a, b, x) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut l = 0;
    let mut r = 1_000_000_001;
    while r - l > 1 {
        let m = (l + r) / 2;
        let c = a * m + b * ((m as f64).log10() as usize + 1);
        if c <= x {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
