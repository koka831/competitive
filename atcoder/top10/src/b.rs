use std::io;

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

fn main() {
    let pat: Vec<String> = ["dream", "dreamer", "erase", "eraser"];
    let s = pat.iter().fold(read::<String>(), |s, x| s.replace(x, ""));
    println!("{}", if s.is_empty() { "YES" } else { "NO" });
}
