use std::io;


fn main() {
    let a = read_one::<String>().chars().collect::<Vec<char>>();
    let b = read_one::<String>().chars().collect::<Vec<char>>();

    let mut flg = "EQUAL";
    if a.len() > b.len() { flg = "GREATER"; }
    else if b.len() > a.len() { flg = "LESS"; }
    else {
        for i in 0..a.len() {
            if a[i] > b[i] { flg = "GREATER"; break; }
            else if a[i] < b[i] { flg = "LESS"; break; }
        }
    }
    println!("{}", flg);
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
