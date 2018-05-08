use std::io;


/// https://beta.atcoder.jp/contests/apc001/tasks/apc001_b
/// sum(b) - sum(a)
fn main() {
    let _n = read_one::<isize>();
    let a = read::<isize>();
    let b = read::<isize>();

    let sum_a = a.iter().sum::<isize>();
    let sum_b = b.iter().sum::<isize>();
    let time = a.iter().zip(b.iter())
        .map(|(x, y)| if x < y { (y - x + 1) / 2 } else { 0 })
        .sum::<isize>();

    if time <= sum_b - sum_a && sum_b >= sum_a {
        println!("Yes");
    } else {
        println!("No");
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
