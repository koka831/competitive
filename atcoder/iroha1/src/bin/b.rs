use std::io;


fn main() {
    let mut s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();
    let k = read_one::<usize>();
    for _ in 0..k {
        let c = s.pop().unwrap();
        s.push(c);
    }

    for c in s {
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
