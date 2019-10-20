use std::io;
use std::cmp;
use std::collections::BinaryHeap;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut ab = Vec::new();
    for _ in 0..n {
        let (a, b) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        ab.push((a, b));
    }
    ab.sort_by(|x, y| (y.0).cmp(&x.0));
    println!("{:?}", ab);

    let mut que = BinaryHeap::new();
    let mut ans: usize = 0;
    let mut idx = 0;
    let mut i = 0;

    while i < m {
        i += 1;
        for j in idx..n {
            if ab[n - j - 1].0 <= cmp::min(m, i) { que.push(ab[n - j - 1].1); }
            else { break; }
            idx = j + 1;
        }
        if let Some(b) = que.pop() {
            ans += b;
        }
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
