use std::io;


fn main() {
    let n = read_one::<usize>();
    if n % 2 != 0 { println!("0"); return; }
    let mut ans: usize = 0;
    let mut x: usize = 10;
    while x <= n {
        ans += (n as f64 / x as f64).floor() as usize;
        x *= 5;
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
