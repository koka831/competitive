use std::io;


fn main() {
    let (l, h) = {
        let i = read::<isize>();
        (i[0], i[1])
    };
    let n = read_one::<usize>();

    for _ in 0..n {
        let a = read_one::<isize>();
        let ans = if a > h { -1 }
        else if a < l { l - a }
        else { 0 };
        println!("{}", ans);
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
