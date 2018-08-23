use std::io;


/// https://beta.atcoder.jp/contests/abc080/tasks/abc080_b
fn main() {
    let n = read_one::<usize>();
    let mut ans = 0;
    let mut x = n;
    while x > 0 {
        ans += x % 10;
        x /= 10;
    }
    if n % ans == 0 { println!("Yes"); }
    else { println!("No"); }
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
