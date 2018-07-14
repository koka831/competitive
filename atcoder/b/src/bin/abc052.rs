use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/abc052/tasks/abc052_b
fn main() {
    let _ = read_one::<usize>();
    let s = read_one::<String>().chars().collect::<Vec<char>>();

    let mut max = 0;
    let mut score = 0;
    for j in 0..s.len() {
        match s[j] {
            'I' => {
                score += 1;
                max = cmp::max(max, score);
            },
            'D' => score -= 1,
            _ => unreachable!(),
        }
    }
    println!("{}", max);
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
