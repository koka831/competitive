use std::io;


fn main() {
    let (a, b) = {
        let i = read::<isize>();
        (i[0], i[1])
    };
    println!("{}", xor(b) ^ xor(a - 1));
}

fn xor(x: isize) -> isize {
    if x % 2 != 0 { return odd_xor(x); }
    odd_xor(x + 1) ^ (x + 1)
}

fn odd_xor(x: isize) -> isize {
    (x + 1) / 2 % 2
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
