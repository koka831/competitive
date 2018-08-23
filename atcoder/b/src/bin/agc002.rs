use std::io;


/// https://beta.atcoder.jp/contests/agc002/tasks/agc002_a
fn main() {
    let (a, b) = {
        let i = read::<isize>();
        (i[0], i[1])
    };
    if a * b <= 0 { println!("Zero"); }
    else if a > 0 { println!("Positive"); }
    else if b < 0 {
        if (b - a) % 2 == 0 { println!("Negative"); }
        else { println!("Positive"); }
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
