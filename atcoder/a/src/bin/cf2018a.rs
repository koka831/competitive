use std::io;


fn main() {
    let a = read_one::<usize>();
    let b = read_one::<usize>();
    let c = read_one::<usize>();
    let s = read_one::<usize>();
    let x = a + b + c;
    if x <= s && s <= x + 3 {
        println!("Yes");
    } else {
        println!("No");
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
