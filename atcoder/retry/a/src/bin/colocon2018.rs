use std::io;


/// https://beta.atcoder.jp/contests/colopl2018-qual/tasks/colopl2018_qual_a
fn main() {
    let (a, b) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let s = read_one::<String>();
    if a <= s.len() && s.len() <= b { println!("YES"); }
    else { println!("NO"); }
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
