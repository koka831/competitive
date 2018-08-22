use std::io;


/// https://beta.atcoder.jp/contests/ddcc2016-qual/tasks/ddcc_2016_qual_a
fn main() {
    let (a, b, c) = {
        let i = read::<f64>();
        (i[0], i[1], i[2])
    };
    println!("{}", c / a * b);
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
