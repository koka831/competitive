use std::io;
use std::cmp;

/// https://beta.atcoder.jp/contests/abc047/tasks/abc047_b
fn main() {
    let (w, h, n) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let mut vec = Vec::new();
    for _ in 0..n {
        let i = read::<usize>();
        vec.push((i[0], i[1], i[2]));
    }

    let mut x_max = w;
    let mut y_max = h;
    let mut x_min = 0;
    let mut y_min = 0;

    while let Some((x_k, y_k, a_k)) = vec.pop() {
        match a_k {
            1 => {
                x_min = cmp::max(x_min, x_k);
            },
            2 => {
                x_max = cmp::min(x_max, x_k);
            },
            3 => {
                y_min = cmp::max(y_min, y_k);
            },
            4 => {
                y_max = cmp::min(y_max, y_k);
            },
            _ => unreachable!(""),
        }
    }
    if x_max >= x_min && y_max >= y_min {
        println!("{}", (x_max - x_min) * (y_max - y_min));
    } else {
        println!("0");
    }
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
