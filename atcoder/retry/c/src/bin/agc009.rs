use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut an = Vec::new();
    let mut bn = Vec::new();

    for _ in 0..n {
        let (a, b) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        an.push(a);
        bn.push(b);
    }
    an.reverse();
    bn.reverse();

    let mut ans: usize = 0;
    for (a, b) in an.iter().zip(bn.iter()) {
        if *a != 0 && *b != 1 {
            ans += (b - (a + ans) % b) % b;
        }
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
