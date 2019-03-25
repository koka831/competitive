use std::io;
use std::cmp;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut sn = Vec::new();
    for _ in 0..n { sn.push(read_one::<usize>()); }

    if sn.contains(&0) { println!("{}", n); return; }

    let mut r = 0;
    let mut prod: usize = 1;
    let mut ans = 0;

    for l in 0..n {
        while r < n && prod * sn[r] <= k {
            prod *= sn[r];
            r += 1;
        }

        ans = cmp::max(ans, r - l);
        if r == l { r += 1; }
        else { prod /= sn[l]; }
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
