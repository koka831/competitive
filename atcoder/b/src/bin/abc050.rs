use std::io;


/// https://beta.atcoder.jp/contests/abc050/tasks/abc050_b
fn main() {
    let _n = read_one::<usize>();
    let t = read::<usize>();
    let m = read_one::<usize>();
    let mut vec = Vec::new();

    for _ in 0..m {
        let i = read::<usize>();
        vec.push(i);
    }

    let sum_t: usize = t.iter().sum();

    for j in 0..m {
        println!("{}", sum_t - t[vec[j][0] - 1] + vec[j][1]);
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
