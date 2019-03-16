use std::io;
use std::collections::BTreeMap;


/// https://beta.atcoder.jp/contests/arc086/tasks/arc086_a
fn main() {
    let (_, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();
    let mut bm = BTreeMap::new();
    for a in an {
        *bm.entry(a).or_insert(0) += 1;
    }

    let mut ans = 0;
    let mut cnt = 0;
    let mut bm = bm.iter().collect::<Vec<_>>();
    bm.sort_by_key(|&(_, v)| -v);
    for v in bm.iter().rev() {
        if bm.len() - cnt <= k { break; }
        cnt += 1;
        ans += *v.1;
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
