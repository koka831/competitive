use std::io;


/// 22:23 ~ 22:53
fn main() {
    let mut n = read_one::<isize>();
    let mut buf = Vec::new();
    if n == 0 { println!("0"); return; }
    while n != 0 {
        if n % 2 != 0 {
            buf.push("1");
            n -= 1;
        } else { buf.push("0"); }
        n /= -2;
    }

    for c in buf.iter().rev() {
        print!("{}", c);
    }
    println!();
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
