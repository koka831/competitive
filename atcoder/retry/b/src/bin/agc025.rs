use std::io;


/// https://beta.atcoder.jp/contests/agc025/tasks/agc025_a
fn main() {
    let n = read_one::<usize>();
    let mut x = n;
    let mut flg = true;
    while x > 1 {
        if x % 10 != 0 { flg = false; }
        x = x / 10;
    }

    if !flg {
        let mut x = n;
        let mut sum = 0;
        while x > 0 {
            sum += x % 10;
            x = x / 10;
        }
        println!("{}", sum);
    } else { println!("10"); }
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
