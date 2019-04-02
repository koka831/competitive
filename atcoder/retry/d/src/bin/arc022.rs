use std::io;
use std::cmp;
use std::collections::HashSet;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();

    let mut ans = 0;
    let mut r = 0;
    let mut flavour = HashSet::new();

    for l in 0..n {
        while r < n && !flavour.contains(&an[r]) {
            flavour.insert(an[r]);
            r += 1;
        }

        ans = cmp::max(ans, r - l);
        if l == r { r += 1; }
        else { flavour.remove(&an[l]); }
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
