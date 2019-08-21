use std::cmp;
use std::io;

fn main() {
    let n = read_one::<usize>();
    let mut vec = Vec::new();
    for _ in 0..n {
        let (x, y, h) = {
            let i = read::<isize>();
            (i[0], i[1], i[2])
        };
        vec.push((x, y, h));
    }

    // avoid x == y == 0
    vec.sort_by_key(|v| v.2);
    vec.reverse();

    for x in 0..101 { for y in 0..101 {
        let h = (x - vec[0].0).abs() + (y - vec[0].1).abs() + vec[0].2;
        if vec.iter()
            .all(|&(a, b, c)| c == cmp::max(h - (x - a).abs() - (y - b).abs(), 0))
        {
            println!("{} {} {}", x, y, h);
            return;
        }
    }}
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
