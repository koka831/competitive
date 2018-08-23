use std::io;


/// https://beta.atcoder.jp/contests/abc084/tasks/abc084_b
fn main() {
    let (a, b) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut flg = true;
    if s[a] != '-' { flg = false; }
    for i in 0..(a + b + 1) {
        if i == a { continue; }
        if !('0' <= s[i] && s[i] <= '9') { flg = false; }
    }

    if flg { println!("Yes"); }
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
