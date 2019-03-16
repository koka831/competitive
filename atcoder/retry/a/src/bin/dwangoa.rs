use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/dwacon2017-prelims/tasks/dwango2017qual_a
/// !!(a && b) == !(!a || !b) == !((n - a) || (n - b)) == a + b - n
fn main() {
    let (n, a, b) = {
        let i = read::<isize>();
        (i[0], i[1], i[2])
    };

    println!("{}", cmp::max(0, a + b - n));
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
