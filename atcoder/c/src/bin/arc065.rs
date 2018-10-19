use std::io;


/// https://beta.atcoder.jp/contests/arc065/tasks/arc065_a
fn main() {
    let s = read_one::<String>();
    let t = vec!["eraser", "erase", "dreamer", "dream"];
    let ans = t.iter().fold(s, |s, x| s.replace(x, ""));
    if ans.is_empty() { println!("YES"); }
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
