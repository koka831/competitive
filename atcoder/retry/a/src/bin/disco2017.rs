use std::io;


/// https://beta.atcoder.jp/contests/ddcc2017-qual/tasks/ddcc2017_qual_a
fn main() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    if s[0] == s[1] && s[1] != s[2] && s[2] == s[3] { println!("Yes"); }
    else { println!("No"); }
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
