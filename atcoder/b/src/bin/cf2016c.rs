use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/code-festival-2016-quala/tasks/codefestival_2016_qualC_b
/// a - 1 - (k - a)
fn main() {
    let (k, _) = {
        let i = read::<isize>();
        (i[0], i[1])
    };
    let a = read::<isize>().into_iter().max().unwrap();
    println!("{}", cmp::max(2 * a - 1 - k, 0))

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
