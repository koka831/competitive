use std::io;
use std::cmp;


fn main() {
    let (n, x) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut an = read::<usize>();
    let mut ans: usize = 0;

    for a in an.iter_mut() {
        if *a > x { ans += *a - x; }
        *a = cmp::min(*a, x);
    }

    for i in 0..n - 1 {
        let total: usize = an[i] + an[i + 1];
        if total > x {
            an[i + 1] = x - an[i];
            ans += total - x;
        }
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
