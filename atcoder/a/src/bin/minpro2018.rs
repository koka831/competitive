use std::io;


/// https://beta.atcoder.jp/contests/yahoo-procon2018-qual/tasks/yahoo_procon2018_qual_a
fn main() {
    let s = read_one::<String>();
    if &s[0..3] == "yah" && &s[3..4] == &s[4..5] { println!("YES"); }
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
