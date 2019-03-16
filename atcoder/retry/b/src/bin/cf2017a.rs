use std::io;


/// https://beta.atcoder.jp/contests/code-festival-2017-quala/tasks/code_festival_2017_quala_b
fn main() {
    let (n, m, k) = {
        let i = read::<isize>();
        (i[0], i[1], i[2])
    };
    let mut flg = false;
    for i in 0..n + 1 { for j in 0..m + 1 {
        if i * (m - j) + j * (n - i) == k { flg = true; break; }
    }}
    if flg { println!("Yes"); }
    else { println!("No"); }
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
