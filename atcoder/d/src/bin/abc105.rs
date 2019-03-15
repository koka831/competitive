use std::io;
use std::collections::HashMap;


/// 23:20 ~ 23:52
fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let an = read::<usize>();
    let mut rem = HashMap::new();
    let mut cs = vec![0; n];
    cs[0] = an[0] % m;
    *rem.entry(cs[0]).or_insert(0) += 1;

    for i in 1..n {
        cs[i] = (cs[i - 1] + an[i]) % m;
        *rem.entry(cs[i]).or_insert(0) += 1;
    }

    let ans = rem.iter()
        .map(|(_, &num)| if num >= 2 { ncr(num, 2) } else { 0 })
        .sum::<usize>();

    println!("{}", ans + rem.get(&0).unwrap_or(&0));
}

fn ncr(n: usize, r: usize) -> usize {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => ncr(n, r - 1) * (n - r + 1) / r,
    }
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
