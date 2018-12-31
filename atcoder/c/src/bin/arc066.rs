use std::io;
use std::collections::HashMap;


/// https://beta.atcoder.jp/contests/arc066/tasks/arc066_a
fn main() {
    const MOD: usize = 1_000_000_007;
    let _ = read_one::<usize>();
    let an = read::<usize>();
    let mut hm = HashMap::new();
    let mut zero = 0;

    for a in an {
        *hm.entry(a).or_insert(0) += 1;
        if a == 0 { zero += 1; }
        if hm.get(&a).unwrap() > &2 {
            println!("0");
            return;
        }
    }

    if zero > 1 {
        println!("0");
        return;
    }

    let mut conb: usize = 1;
    for v in hm.values() {
        conb = conb * v;
        conb = conb % MOD;
    }
    println!("{}", conb);
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
