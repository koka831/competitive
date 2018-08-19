use std::io;


fn main() {
    let c1 = read_one::<String>().chars().collect::<Vec<char>>();
    let c2 = read_one::<String>().chars().collect::<Vec<char>>();
    let c3 = read_one::<String>().chars().collect::<Vec<char>>();

    println!("{}{}{}", c1[0], c2[1], c3[2]);
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
