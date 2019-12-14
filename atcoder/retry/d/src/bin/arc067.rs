use std::io;
use std::cmp;


fn main() {
    let (n, a, b) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let xn = read::<usize>();
    let mut ans = 0;
    for i in 1..n {
        ans += cmp::min(b, (xn[i] - xn[i - 1]) * a);
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
