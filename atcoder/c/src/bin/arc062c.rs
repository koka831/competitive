use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut t = 1.0;
    let mut a = 1.0;
    for _ in 0..n {
        let (t_i, a_i) = {
            let i = read::<f32>();
            (i[0], i[1])
        };
        let _n = cmp::max((t / t_i).ceil(), (a / a_i).ceil());
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
