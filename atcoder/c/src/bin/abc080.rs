use std::io;


/// https://beta.atcoder.jp/contests/abc080/tasks/abc080_c
/// Shopping Street
/// 2^10 通りを全探索
fn main() {
    let n = read_one::<usize>();
    let mut fs = Vec::new();
    let mut ps = Vec::new();

    for _ in 0..n { fs.push(read::<isize>()); }
    for _ in 0..n { ps.push(read::<isize>()); }

    let mut ans = ::std::isize::MAX * -1;
    for i in 1..1 << 10 {
        let mut tmp = 0;
        for k in 0..n {
            let mut cnt = 0;
            for j in 0..10 {
                if (i & 1 << j) != 0 {
                    if fs[k][j] == 1 { cnt += 1; }
                }
            }
            tmp += ps[k][cnt];
        }
        if ans < tmp { ans = tmp; }
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
