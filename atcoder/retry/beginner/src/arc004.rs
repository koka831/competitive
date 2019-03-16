#![allow(unused_imports)]
use std::io;
use std::cmp;


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

fn dist(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

fn main() {
    let n = read_one::<usize>();
    let mut vec: Vec<Vec<f32>> = Vec::new();

    for _i in 0..n {
        vec.push(read::<f32>());
    }

    let mut max = 0f32;

    for j in 0..n { for k in 0.. n {
        let d = dist(&vec[j], &vec[k]);
        max = if max < d {
            d
        } else { max }
    }}

    println!("{}", max);
}
