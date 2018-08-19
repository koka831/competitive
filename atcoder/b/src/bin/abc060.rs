use std::io;


fn main() {
    let (a, b, c) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut flg = false;
    for i in 0..b {
        if a * (i + 1) % b == c { flg = true; }
    }
    if flg { println!("YES"); }
    else { println!("NO"); }
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
