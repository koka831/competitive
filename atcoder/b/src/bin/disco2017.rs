use std::io;


/// https://beta.atcoder.jp/contests/ddcc2017-qual/tasks/ddcc2017_qual_b
fn main() {
    let (a, b, c, d) = {
        let i = read::<usize>();
        (i[0], i[1], i[2], i[3])
    };
    println!("{}", a * 1728 + b * 144 + c * 12 + d);
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
