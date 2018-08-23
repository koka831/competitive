use std::io;
use std::collections::HashMap;


/// https://beta.atcoder.jp/contests/arc082/tasks/arc082_a
fn main() {
    let _ = read_one::<usize>();
    let an = read::<isize>();
    let mut hm = HashMap::new();
    let am = an.clone().iter().map(|i| i - 1).collect::<Vec<_>>();
    let ap = an.clone().iter().map(|i| i + 1).collect::<Vec<_>>();

    for a in an {
        *hm.entry(a).or_insert(0) += 1;
    }

    for a in am {
        *hm.entry(a).or_insert(0) += 1;
    }

    for a in ap {
        *hm.entry(a).or_insert(0) += 1;
    }

    let mut v = hm.iter().collect::<Vec<_>>();
    v.sort_by_key(|&(_, v)| -v);

    println!("{:?}", v[0].1);
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
