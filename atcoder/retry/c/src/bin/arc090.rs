use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/arc090/tasks/arc090_a
fn main() {
    let n = read_one::<usize>();
    let mut an = Vec::new();
    for _ in 0..2 {
        an.push(read::<usize>());
    }
    let mut ans = 0;
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..n + 1 {
            if j <= i { cnt += an[0][j]; }
            else { cnt += an[1][j - 1]; }
        }
        ans = cmp::max(ans, cnt);
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
