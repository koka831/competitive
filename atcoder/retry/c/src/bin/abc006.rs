use std::io;

/// https://beta.atcoder.jp/contests/abc006/tasks/abc006_3
/// 2a + 3b + 4c = m
/// a + b + c = n
/// 2b = a + b
/// b = 0, 1
fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    for i in 0..n + 1 {
        // c == 0
        if 2 * i + 4 * (n - i) == m {
            println!("{} 0 {}", i, n - i);
            return;
        } else if i < n && 2 * i + 3 + 4 * (n - i - 1) == m {
            println!("{} 1 {}", i, n - i - 1);
            return;
        }
    }
    println!("-1 -1 -1");
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
