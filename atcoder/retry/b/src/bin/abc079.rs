use std::io;


/// https://beta.atcoder.jp/contests/abc079/tasks/abc079_b
fn main() {
    let n = read_one::<usize>();
    let mut lucas = vec![0u64; n + 1];
    lucas[0] = 2;
    lucas[1] = 1;
    for i in 2..(n + 1) {
        lucas[i] = lucas[i - 1] + lucas[i - 2];
    }
    println!("{}", lucas[n]);
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
