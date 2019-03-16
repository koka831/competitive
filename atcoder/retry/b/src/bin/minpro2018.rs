use std::io;


/// https://beta.atcoder.jp/contests/yahoo-procon2018-qual/tasks/yahoo_procon2018_qual_b
fn main() {
    let (x, k) = {
        let i = read::<u32>();
        (i[0], i[1])
    };

    let pow = 10u32.pow(k);
    println!("{}", (x / pow) * pow + pow);
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
