use std::io;


/// https://beta.atcoder.jp/contests/abc049/tasks/abc049_b
fn main() {
    let (h, _) = {
        let i = read::<isize>();
        (i[0], i[1])
    };

    let mut vec = Vec::new();

    for _ in 0..h {
        let i = read_one::<String>();
        let j = i.clone();
        vec.push(i);
        vec.push(j);
    }

    let mut vec = vec.into_iter().rev().collect::<Vec<String>>();

    while let Some(s) = vec.pop() {
        println!("{}", s);
    }
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
