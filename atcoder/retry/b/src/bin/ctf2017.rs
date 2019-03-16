use std::io;


/// https://beta.atcoder.jp/contests/code-thanks-festival-2017-open/tasks/code_thanks_festival_2017_b
fn main() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    for i in 0..s.len() {
        let mut tmp = s.clone();
        for j in 1..i + 1 { tmp.push(s[i - j]); }
        if is_palindrome(&tmp) { println!("{}", i); break; }
    }
}

fn is_palindrome(s: &[char]) -> bool {
    for i in 0..s.len() {
        if s[i] != s[s.len() - 1 - i] { return false; }
    }
    true
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
