use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut tn = Vec::new();
    let mut an = Vec::new();
    for _ in 0..n {
        let (t, a) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        tn.push(t);
        an.push(a);
    }

    let mut t = 1usize;
    let mut a = 1usize;

    for i in 0..n {
        let x: usize = cmp::max(
            (t + tn[i] - 1) / tn[i],
            (a + an[i] - 1) / an[i]
        );
        t = tn[i] * x;
        a = an[i] * x;
    }
    println!("{}", t + a);
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
