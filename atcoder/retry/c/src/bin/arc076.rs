use std::io;


/// https://beta.atcoder.jp/contests/arc076/tasks/arc076_a
fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let md = 1_000_000_000 + 7;
    if (n as isize - m as isize).abs() > 1 { println!("0"); }
    else if (n as isize - m as isize).abs() == 1 { println!("{}", (fact(n) * fact(m)) % md); }
    else if n == m { println!("{}", 2 * ((fact(n) * fact(m)) % md) % md); }
}
 
fn fact(n: usize) -> usize {
    let m = 1_000_000_000 + 7;
    match n {
        1 => 1,
        _ => n * fact(n - 1) % m,
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
