use std::io;


fn main() {
    let s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();

    let n = s.len();

    let mut cost = 0;
    for i in 1..n + 1 {
        if s[i - 1] == 'U' {
            cost += (n - i) + 2 * (i - 1);
        } else {
            cost += (i - 1) + 2 * (n - i);
        }
    }

    println!("{}", cost);
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
