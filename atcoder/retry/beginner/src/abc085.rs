#![allow(unused_imports)]
use std::io;
use std::cmp;


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


fn main() {
    let (n, y) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut flg = 0;

    'outer: for j in 0..(n + 1) {
        'inner: for k in 0..(n - j + 1) {
            if 10000 * j + 5000 * k + 1000 * (n - j - k) == y {
                println!("{} {} {}", j, k, n - j - k);
                flg = 1;
                break 'outer;
            }
        }
    }
    if flg == 0 {
        println!("-1 -1 -1");
    }
}
