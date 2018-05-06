use std::io;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
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

fn main() {
    let s: Vec<char> = read_one::<String>().chars().collect();
    let mut buf = String::new();

    for c in s {
        match c {
            'B' => {
                if buf.len() > 0 {
                    buf.pop();
                }
            },
            _ => buf.push(c),
        }
    }

    let ans = buf.chars()
        .fold(String::from(""), |a, b| a + &b.to_string());

    println!("{}", ans);
}
