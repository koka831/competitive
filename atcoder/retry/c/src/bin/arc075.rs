use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut sn = Vec::new();
    for _ in 0..n { sn.push(read_one::<usize>()); }
    let s = sn.iter().sum::<usize>();
    
    if sn.iter().all(|s| s % 10 == 0) {
        println!("0");
        return;
    }

    let min = sn.iter()
        .filter(|s| *s % 10 != 0)
        .min().unwrap();
    if s % 10 == 0 {
        println!("{}", s - min);
    } else {
        println!("{}", s);
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
