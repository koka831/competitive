use std::io;


fn main() {
    let n = read_one::<usize>();
    for i in 1..3501 { for j in 1..3501 {
        if i * j % n != 0 { continue; }
        let denom = i * j;
        let numer = 4 * i * j / n;
        if numer < i + j { continue; }
        let numer = 4 * i * j / n - i - j;
        if numer == 0 { continue; }
        if denom % numer == 0 { println!("{} {} {}", i, j, denom / numer); return; }
    }}
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
