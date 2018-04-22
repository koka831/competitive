use std::io;

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
    let _n = read_one::<usize>();
    let cs = read_one::<String>();

    let mut count = [0; 4];

    for c in cs.as_bytes().iter() { count[(c - b'1') as usize] += 1; }

    println!("{} {}", count.iter().max().unwrap(), count.iter().min().unwrap());
}
