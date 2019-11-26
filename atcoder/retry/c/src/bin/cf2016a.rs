use std::io;


fn main() {
    let mut s = read_one::<String>().chars().collect::<Vec<_>>();
    let mut k = read_one::<usize>();
    let n = s.len();
    for i in 0..n {
        if s[i] == 'a' { continue; }
        let idx = (s[i] as u8 - b'a') as usize;
        // println!("idx: {}", idx);
        if 26 - idx <= k { k -= 26 - idx; s[i] = 'a'; }
    }

    // consume k
    let idx = (s[n - 1] as u8 - b'a') as usize;
    let new = (idx + k) % 26;
    s[n - 1] = (new as u8 + b'a') as char;
    for i in 0..n {
        if i == n - 1 { println!("{}", s[i]); }
        else { print!("{}", s[i]); }
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
