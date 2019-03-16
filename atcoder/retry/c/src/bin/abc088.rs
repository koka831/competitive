use std::io;


/// https://beta.atcoder.jp/contests/abc088/tasks/abc088_c
/// Takahashi's Information
/// an = pn + x
/// bn = qn - x
fn main() {
    let c1 = read::<usize>();
    let c2 = read::<usize>();
    let c3 = read::<usize>();
    let c = vec![c1, c2, c3];
    let mut flg = true;
    for i in 0..3 { for j in 0..3 {
    for k in 0..3 { for l in 0..3 {
        if c[i][k] + c[j][l] != c[i][l] + c[j][k] { flg = false; }
    }}
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
