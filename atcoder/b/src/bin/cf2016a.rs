use std::io;


/// https://beta.atcoder.jp/contests/code-festival-2016-quala/tasks/codefestival_2016_qualA_b
fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();
    let mut cnt = 0;
    for i in 0..n {
        if an[an[i] - 1] == i + 1 { cnt += 1; }
    }
    println!("{}", cnt / 2);
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
