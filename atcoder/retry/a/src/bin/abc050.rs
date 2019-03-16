use std::io;


fn main() {
    let i = read::<String>();
    let a = i[0].parse::<isize>().unwrap();
    let b = i[2].parse::<isize>().unwrap();

    match &*i[1] {
        "+" => { println!("{}", a + b); },
        "-" => { println!("{}", a - b); },
        _   => unreachable!(),
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
