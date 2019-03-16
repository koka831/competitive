use std::io;


/// https://beta.atcoder.jp/contests/agc006/tasks/agc006_a
fn main() {
    let _ = read_one::<usize>();
    let s = read_one::<String>();
    let t = read_one::<String>();
    let mut ans = s.clone();
    let mut buf = t.clone();
    let mut post = "".to_string();
    while buf.len() > 0 {
        if s.contains(&buf) { ans += &post; break; }
        post.insert(0, buf.pop().unwrap());
    }
    if buf.len() == 0 { ans += &t; }
    println!("{}", ans.len());
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
