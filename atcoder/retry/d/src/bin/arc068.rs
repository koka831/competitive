use std::io;
use std::collections::HashMap;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();
    let mut hm = HashMap::new();
    for a in an {
        *hm.entry(a).or_insert(0) += 1;
    }

    let mut cnt = n;
    let mut co = 0;
    let mut hm = hm.values().collect::<Vec<_>>();
    hm.sort();
    hm.reverse();
    for &v in hm {
        let mut x = v;
        if co > 0 && x >= 2 { co -= 1; x -= 1; }
        if x >= 3 {
            let t = (x - 1) / 2;
            // println!("t: {}", t);
            cnt -= t * 2; x -= t * 2;
        }
        if x == 2 { cnt -= 2; co += 1; }
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
