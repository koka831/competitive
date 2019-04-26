use std::io;


// const MOD: usize = 1e9 as usize + 7;

fn main() {
    let (n, _) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut f_n = Vec::new();
    for _ in 0..n { f_n.push(read_one::<usize>()); }
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
