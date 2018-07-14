use std::io;


fn main() {
    let (_, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut a = read::<usize>();
    a.sort();

    let mut ans = 0;
    let mut cnt = 0;
    for j in 0..k {
        ans += a[j] + cnt;
        cnt += 1;
    }
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
