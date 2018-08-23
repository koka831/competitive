use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/arc059/tasks/arc059_a
fn main() {
    let _ = read_one::<usize>();
    let an = read::<isize>();
    let mut ans = 10000 * 10000;
    for i in 0..201 {
        let sum = an.iter().fold(0, |s, x| s + (x - (i - 100)) * (x - (i - 100)));
        ans = cmp::min(ans, sum);
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
