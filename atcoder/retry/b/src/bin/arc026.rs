use std::io;
use std::cmp::Ordering;


fn main() {
    let n = read_one::<usize>();
    let fs = factors(n);
    let s = fs.iter().sum::<usize>();
    let ans = match (n * 2).cmp(&s) {
        Ordering::Equal => "Perfect",
        Ordering::Greater => "Deficient",
        Ordering::Less => "Abundant"
    };
    println!("{}", ans);
}

fn factors(n: usize) -> Vec<usize> {
    let mut fs = Vec::new();
    for i in 1..(n as f64).sqrt().ceil() as usize {
        if n % i == 0 {
            fs.push(i);
            fs.push(n / i);
        }
    }
    fs
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
