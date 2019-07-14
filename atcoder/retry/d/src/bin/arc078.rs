use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut g = vec![Vec::new(); n];
    for _ in 1..n {
        let (a, b) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        g[a].push(b);
        g[b].push(a);
    }

    println!("{:?}", g);
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
