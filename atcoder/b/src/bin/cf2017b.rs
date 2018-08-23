use std::io;
use std::collections::HashMap;


/// https://beta.atcoder.jp/contests/code-festival-2017-quala/tasks/code_festival_2017_qualb_b
fn main() {
    let _ = read_one::<usize>();
    let dn = read::<usize>();
    let _ = read_one::<usize>();
    let tm = read::<usize>();

    let mut hmd = HashMap::new();
    let mut hmt = HashMap::new();
    for d in &dn {
        *hmd.entry(d).or_insert(0) += 1;
    }
    for t in &tm {
        *hmt.entry(t).or_insert(0) += 1;
    }

    let mut flg = true;
    for t in hmt.keys() {
        if *hmd.get(t).unwrap_or(&0) < *hmt.get(t).unwrap() { flg = false; }
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
