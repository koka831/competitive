use std::io;


fn main() {
    let (n, ma, mb) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let mut an = Vec::new();
    let mut bn = Vec::new();
    let mut cn = Vec::new();

    for _ in 0..n {
        let (a, b, c) = {
            let i = read::<usize>();
            (i[0], i[1], i[2])
        };
        an.push(a);
        bn.push(b);
        cn.push(c);
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
