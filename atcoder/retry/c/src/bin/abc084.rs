use std::io;


/// https://beta.atcoder.jp/contests/abc084/tasks/abc084_c
/// Special Trains
fn main() {
    let n = read_one::<usize>();
    let mut csf = Vec::new();
    for _ in 1..n {
        let i = read::<usize>();
        csf.push((i[0], i[1], i[2]));
    }

    for i in 0..n {
        let mut t = 0;
        for j in i..n - 1 {
            if t < csf[j].1 { t = csf[j].1; }
            else if t % csf[j].2 != 0 {
                t += csf[j].2 - t % csf[j].2;
            }
            t += csf[j].0;
        }
        println!("{}", t);
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
