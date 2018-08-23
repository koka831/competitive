use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/arc078/tasks/arc078_a
fn main() {
    let n = read_one::<usize>();
    let an = read::<isize>();
    let all = an.iter().sum::<isize>();
    let mut ans = ::std::isize::MAX;
    let mut sum = 0;
    for i in 0..n - 1 {
        sum += an[i];
        ans = cmp::min(ans, (all - 2 * sum).abs());
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
