use std::io;
use std::cmp;


fn main() {
    let (n, mut m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ans:usize = 0;

    let a = cmp::min(n, m / 2);
    ans += a;
    m -= a * 2;
    ans += m / 4;

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
