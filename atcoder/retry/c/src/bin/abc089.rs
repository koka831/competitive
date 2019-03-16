use std::io;


/// https://beta.atcoder.jp/contests/abc089/tasks/abc089_c
/// March
/// multiply overflow
fn main() {
    let n = read_one::<usize>();
    let mut m: u64 = 0;
    let mut a: u64 = 0;
    let mut r: u64 = 0;
    let mut c: u64 = 0;
    let mut h: u64 = 0;
    for _ in 0..n {
        let s = read_one::<String>().chars().collect::<Vec<char>>();
        match s[0] {
            'M' => m += 1,
            'A' => a += 1,
            'R' => r += 1,
            'C' => c += 1,
            'H' => h += 1,
            _ => {},
        }
    }
    // 5C3 -> 10
    let mut ans = 0;
    ans += m * a * r;
    ans += m * a * c;
    ans += m * a * h;
    ans += m * r * c;
    ans += m * r * h;
    ans += m * c * h;
    ans += a * r * c;
    ans += a * r * h;
    ans += a * c * h;
    ans += r * c * h;

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
