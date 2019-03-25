use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let an = read::<isize>();

    let mut aoki = 0;
    let mut chok = 0;

    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            let a = an[cmp::min(i, j)..cmp::max(i, j)].iter().enumerate()
                .filter(|&(i, _)| i % 2 != 0)
                .map(|(_, x)| x)
                .sum::<isize>();
            if a > aoki {
                aoki = a;
                chok = an[cmp::min(i, j)..cmp::max(i, j)].iter().sum::<isize>();
            }
        }
    }

    println!("{}", chok);
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
