use std::io;


/// https://beta.atcoder.jp/contests/code-festival-2017-quala/tasks/code_festival_2017_qualc_b
fn main() {
    let n = read_one::<u32>();
    let an = read::<usize>();
    let mut odd = 1;
    for a in an {
        if a % 2 == 0 { odd *= 2; }
    }
    println!("{}", 3u32.pow(n) - odd);
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
