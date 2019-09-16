use std::io;
use std::cmp;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let vn = read::<isize>();
    let mut ans: isize = 0;
    let time = cmp::min(n, k) + 1;
    for l in 1..time { for r in 0..(time - l) {
        let mut score = vn[0..l].iter().sum::<isize>() + vn[(n - r)..n].iter().sum::<isize>();
        let mut arr = Vec::from(&vn[0..l]);
        arr.extend_from_slice(&vn[(n - r)..n]);
        arr.sort();
        let dis = k - l - r;
        for i in 0..cmp::min(dis, arr.len()) {
            if arr[i] < 0 { score -= arr[i]; }
        }
        ans = cmp::max(ans, score);
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
