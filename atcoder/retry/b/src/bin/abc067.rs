use std::io;


/// https://beta.atcoder.jp/contests/abc067/tasks/abc067_b
fn main() {
    let (_n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vec = read::<usize>();
    vec.sort();
    vec.reverse();
    println!("{}", vec.iter().take(k).sum::<usize>());
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
