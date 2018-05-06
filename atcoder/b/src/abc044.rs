use std::io;
use std::collections::HashMap;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
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

fn main() {
    let w  = read_one::<String>();

    let mut hm = HashMap::new();

    for c in w.chars() {
        let i = hm.entry(c).or_insert(0);
        *i += 1;
    }

    match hm.values().find(|&c| c % 2 != 0) {
        Some(_) => println!("No"),
        None => println!("Yes"),
    }
}
