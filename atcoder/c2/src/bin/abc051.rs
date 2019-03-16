use std::io;


// 22:13 ~ 22:20
fn main() {
    let (a, b, c, d) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3])
    };

    for _ in 0..(a - c).abs() {
        println!("L");
    }

    for _ in 0..(b - d).abs() {
        println!("U");
    }

    for _ in 0..(a - c).abs() {
        println!("R");
    }

    for _ in 0..(b - d).abs() {
        println!("D");
    }

    println!("D");

    for _ in 0..(a - c).abs() + 1 {
        println!("L");
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
