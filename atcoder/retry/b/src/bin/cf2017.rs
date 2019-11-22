use std::io;


fn main() {
    let s = read_one::<String>().chars().collect::<Vec<_>>();
    let mut cnt = vec![0; 3];
    for c in s {
        match c {
            'a' => cnt[0] += 1,
            'b' => cnt[1] += 1,
            'c' => cnt[2] += 1,
            _ => unreachable!(),
        }
    }
    let max = cnt.iter().max().unwrap();
    let min = cnt.iter().min().unwrap();
    if max - min > 1 { println!("NO"); }
    else { println!("YES"); }
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
