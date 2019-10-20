use std::io;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let ak = read::<usize>();
    let mut sn = Vec::new();
    for _ in 0..n { sn.push(read_one::<String>()); }
    // for each |s_max| s.t. all s of sn \in ak has the same prefix
    let s_max = sn.iter()
        .map(|s| s.len()).max().unwrap();
    let mut ans = 0;
    for i in 1..s_max {
        let mut flg = true;
        if sn.iter().all(|s| s[..i] == sn[0][..i]) { ans = i; }
    }

    if ans == 0 { println!("-1"); }
    else { println!("{}", ans); }
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
