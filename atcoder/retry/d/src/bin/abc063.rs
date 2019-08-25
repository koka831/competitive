use std::io;

fn main() {
    let (n, a, b) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let mut hn = Vec::new();
    for _ in 0..n { hn.push(read_one::<usize>()); }

    let valid = |t: usize| -> bool {
        let mut cnt: usize = 0;
        for &h in hn.iter() {
            if h > t * b {
                cnt += ((h - t * b) as f64 / (a - b) as f64).ceil() as usize;
            }
        }
        cnt <= t
    };

    let mut l = 0;
    let mut r = *hn.iter().max().unwrap() as isize;
    while r - l > 1 {
        let m = (l + r) / 2;
        if valid(m as usize) {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r);
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
