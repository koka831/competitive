use std::io;


fn main() {
    let s = read_one::<String>()
        .chars().collect::<Vec<char>>();
    let mut flg = true;
    for i in 0..s.len() {
        if s[i] == ' ' {
            if s[i - 1] != s[i + 1] { flg = false; }
        }
    }

    if flg { println!("YES"); }
    else { println!("NO"); }
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
