use std::io;

/// https://beta.atcoder.jp/contests/agc005/tasks/agc005_a
fn main() {
    let x = read_one::<String>();
    let mut buf = Vec::new();

    for c in x.chars() {
        match c {
            'S' => buf.push(c),
            'T' => {
                if let Some(&'S') = buf.last() {
                    buf.pop();
                } else {
                    buf.push(c);
                }
            },
            _ => (),
        }
    }

    println!("{}", buf.len());
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
