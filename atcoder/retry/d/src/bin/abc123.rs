use std::io;
use std::cmp;

fn main() {
    
    let (x, y, _, k) = {
        let i = read::<usize>();
        (i[0], i[1], i[2], i[3])
    };

    let ax = read::<usize>();
    let by = read::<usize>();
    let cz = read::<usize>();
    let mut ab = Vec::new();
    for a in ax.iter() { for b in by.iter() {
        ab.push(a + b);
    }}

    ab.sort();
    ab.reverse();
    let mut abc = Vec::new();
    for i in 0..cmp::min(k, x * y) { for c in cz.iter() {
        abc.push(ab[i] + c);
    }}

    abc.sort();
    abc.reverse();
    for i in 0..k {
        println!("{}", abc[i]);
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
