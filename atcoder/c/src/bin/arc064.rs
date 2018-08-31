use std::io;


/// https://beta.atcoder.jp/contests/arc064/tasks/arc064_a
/// Boxes and Candies
fn main() {
    let (n, x) = {
        let i = read::<usize>();
        (i[0], i[1] as i64)
    };
    let mut an = read::<i64>();
    println!("{}", an.len());
    let mut ans = 0;
    for i in 0..n - 1 {
        let total = an[i] + an[i + 1];
        if total > x {
            an[i + 1] = x - an[i];
            ans += total - x;
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
