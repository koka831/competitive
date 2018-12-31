use std::io;
use std::collections::HashMap;


fn main() {
    const MOD: usize = 1_000_000_007;
    let n = read_one::<usize>();

    let mut hm = HashMap::new();
    for i in 1..n + 1 {
        let mut x = i;
        while let Some(f) = divide(x) {
            x /= f;
            *hm.entry(f).or_insert(0) += 1;
        }
        *hm.entry(x).or_insert(0) += 1;
    }

    let mut ans: usize = 1;
    for (k, v) in hm.iter() {
        if k == &1 { continue; }
        ans = ans * (*v + 1);
        ans = ans % MOD;
    }
    println!("{}", ans);

}

fn divide(n: usize) -> Option<usize> {
    for i in 2..(n as f64).sqrt().ceil() as usize + 2 {
        if n % i == 0 {
            return Some(i);
        }
    } 
    return None;
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
