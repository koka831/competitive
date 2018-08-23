use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/soundhound2018/tasks/soundhound2018_b
fn main() {
    let (_, l, r) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let an = read::<usize>();
    let mut am = Vec::new();
    for a in an {
        am.push(cmp::min(r, cmp::max(a, l)).to_string());
    }

    println!("{}", am.join(" "));
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
