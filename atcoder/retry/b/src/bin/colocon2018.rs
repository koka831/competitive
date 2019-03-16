use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/colopl2018-qual/tasks/colopl2018_qual_b
fn main() {
    let (n, x) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut tn = Vec::new();
    for _ in 0..n { tn.push(read_one::<usize>()); }
    let mut ans = 0;
    for i in 0..s.len() {
        if s[i] == '0' { ans += tn[i]; }
        if s[i] == '1' { ans += cmp::min(x, tn[i]); }
    }

    println!("{}", ans);
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
