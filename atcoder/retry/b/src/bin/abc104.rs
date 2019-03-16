use std::io;


/// https://beta.atcoder.jp/contests/abc104/tasks/abc104_b
fn main() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut flg = true;
    if s[0] != 'A' { flg = false; }
    if !(s.contains(&'C') && s[1] != 'C' && s[s.len() - 1] != 'C') { flg = false; }
    if s.iter().filter(|c| c.is_lowercase()).count() != s.len() - 2 { flg = false; }
    if flg { println!("AC"); }
    else { println!("WA"); }
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
