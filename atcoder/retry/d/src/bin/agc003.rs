use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut an = vec![0; n];
    for i in 0..n { an[i] = read_one::<usize>(); }
    let mut cnt: usize = an[0] / 2;
    an[0] = an[0] % 2;
    for i in 1..n {
        let d = cmp::min(an[i - 1], an[i]);
        cnt += d + (an[i] - d) / 2;
        an[i] = (an[i] - d) % 2;
    }

    println!("{}", cnt);
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
