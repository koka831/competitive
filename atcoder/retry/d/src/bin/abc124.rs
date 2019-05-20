use std::io;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let s = read_one::<String>()
        .chars()
        .map(|c| (c as u8) - '0' as u8)
        .collect::<Vec<u8>>();

    println!("{:?}", s);
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
