use std::io;
use std::collections::HashMap;


/// https://beta.atcoder.jp/contests/arc087/tasks/arc087_a
/// **only remove**
fn main() {
    let _ = read_one::<usize>();
    let an = read::<isize>();
    let mut cnt = 0;
    let mut hm = HashMap::new();
    for a in an {
        *hm.entry(a).or_insert(0) += 1;
    }

    for a in hm.keys() {
        let &v = hm.get(a).unwrap();
        cnt += if v < *a { v } else { v - *a };
    }
    println!("{}", cnt);
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
