use std::io;


/// https://beta.atcoder.jp/contests/abc086/tasks/abc086_b
fn main() {
    let (a, b) = {
        let i = read::<u32>();
        (i[0], i[1])
    };
    let sq = a * 10u32.pow(digit(b)) + b;
    for i in 0..317 {
        if i * i == sq { println!("Yes"); return; }
    }
    println!("No");
}

fn digit(n: u32) -> u32 {
    let mut x = n;
    let mut dig = 0;
    while x > 0 {
        dig += 1;
        x = x / 10;
    }
    dig
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
