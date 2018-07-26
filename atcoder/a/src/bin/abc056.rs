use std::io;


fn main() {
    let i = read::<String>();

    let ans = match (&*i[0], &*i[1]) {
        ("H", "H") => true,
        ("D", "D") => true,
        ("H", "D") => false,
        ("D", "H") => false,
        _ => unreachable!(),
    };

    if ans { println!("H"); }
    else { println!("D"); }
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
