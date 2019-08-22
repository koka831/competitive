use std::io;
use std::cmp;


fn main() {
    let (a, b) = {
        let i = read::<isize>();
        (i[0], i[1])
    };

    let mut ans = 1_000_000;
    for i in -9..10 { for j in -9..10 { for k in -9..10 {
        let x = 10 * i + j * 5 + k;
        if (b - a) == x { ans = cmp::min(ans, i.abs() + j.abs() + k.abs()); }
    }}}
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
