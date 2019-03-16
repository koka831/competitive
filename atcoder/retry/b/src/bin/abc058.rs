use std::io;


fn main() {
    let o = read_one::<String>().chars().collect::<Vec<char>>();
    let e = read_one::<String>().chars().collect::<Vec<char>>();
    let mut pass = Vec::new();
    for i in 0..(o.len() + e.len()) {
        if i % 2 == 0 { pass.push(o[i/2]); }
        else { pass.push(e[(i-1)/2]); }
    }
    for c in pass {
        print!("{}", c);
    }
    println!();
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
