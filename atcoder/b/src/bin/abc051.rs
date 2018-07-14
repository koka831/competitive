use std::io;


/// https://beta.atcoder.jp/contests/abc051/tasks/abc051_b
fn main() {
    let (k, s) = {
        let i = read::<isize>();
        (i[0], i[1])
    };

    let mut ans = 0;
    for i in 0..(k + 1) { for j in 0..(k + 1) {
        let z = s - i - j;
        if 0 <= z && z <= k { ans += 1; }
    }}

    println!("{}", ans);
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
