use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/abc080/tasks/abc080_c
fn main() {
    let n = read_one::<usize>();
    let mut fs = Vec::new();
    let mut ps = Vec::new();

    for _ in 0..n { fs.push(read::<isize>()); }
    for _ in 0..n { ps.push(read::<isize>()); }

    let ans: isize = fs.iter().zip(ps.iter())
        .map(|(f, p)| {
            let least_i = p.iter().max().unwrap();
            let sum: isize = f.iter().zip(p.iter())
                .map(|(f_i, p_i)| {
                    if *f_i == 1 { *p_i } else { 0 }
                })
                .sum();
            cmp::max(*least_i, sum)
        })
        .sum();

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
