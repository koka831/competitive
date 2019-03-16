use std::io;
use std::collections::BTreeMap;


/// https://beta.atcoder.jp/contests/arc081/tasks/arc081_a
fn main() {
    let _n = read_one::<usize>();
    let an = read::<isize>();
    let mut bm = BTreeMap::new();

    for a in an {
        *bm.entry(a).or_insert(0) += 1;
    }

    let mut vec = bm.into_iter().filter(|&(_, v)| v >= 2).collect::<Vec<_>>();
    vec.sort_by_key(|&(k, _)| -k);
    if vec.len() < 2 { println!("0"); }
    else if vec[0].1 >= 4 { println!("{}", vec[0].0 * vec[0].0); }
    else { println!("{}", vec[0].0 * vec[1].0)}
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
