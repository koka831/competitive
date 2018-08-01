use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut t = Vec::new();
    for _ in 0..n {
        t.push(read_one::<usize>());
    }

    let mut ans = 200;
    for i in 0..1 << n {
        let mut lhv = 0;
        let mut rhv = 0;
        for j in 0..n {
            if (i & 1 << j) != 0 {
                lhv += t[j];
            } else {
                rhv += t[j];
            }
        } 
        ans = cmp::min(ans, cmp::max(lhv, rhv));
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
