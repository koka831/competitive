use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/dwacon2018-prelims/tasks/dwacon2018_prelims_b
/// depth
fn main() {
    let s = read_one::<String>();

    let mut cnt_5 = 0;
    let mut max_len = 0;
    let mut seq = 0;
    let mut prev = '1';

    for c in s.chars().rev() {
        match c {
            '5' => cnt_5 += 1,
            '2' => cnt_5 -= 1,
            _ => unreachable!(),
        }

        if cnt_5 < 0 { println!("-1"); return; }

        if prev != c {
            seq = 0;
        }
        prev = c;
        seq += 1;
        max_len = cmp::max(max_len, seq);
    }

    if cnt_5 != 0 { println!("-1"); return; }

    println!("{}", max_len);

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
