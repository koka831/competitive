use std::io;


fn main() {
    let c1j = read_one::<String>().chars().collect::<Vec<char>>();
    let c2j = read_one::<String>().chars().collect::<Vec<char>>();
    if c1j[0] == c2j[2] && c1j[2] == c2j[0] && c1j[1] == c2j[1] {
        println!("YES");
    } else {
        println!("NO");
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
