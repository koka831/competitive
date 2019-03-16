use std::io;


/// https://beta.atcoder.jp/contests/yahoo-procon2017-qual/tasks/yahoo_procon2017_qual_a
fn main() {
    let mut s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut ans = "yahoo".to_string().chars().collect::<Vec<char>>();
    s.sort();
    ans.sort();
    if &s == &ans { println!("YES"); }
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
