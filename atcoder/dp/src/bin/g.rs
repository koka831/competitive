use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut xm = Vec::new();
    let mut ym = Vec::new();
    for _ in 0..m {
        let (x_m, y_m) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        xm.push(x_m);
        ym.push(y_m);
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
