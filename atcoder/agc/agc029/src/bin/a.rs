use std::io;


fn main() {
    let mut ans: u64 = 0;
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut cnt: u64 = 0;
    for i in 0..s.len() {
        if s[i] == 'B' { cnt += 1; }
        if s[i] == 'W' { ans += cnt; }
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
