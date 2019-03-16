use std::io;


fn main() {
    let s = read_one::<String>().chars()
        /*
        .filter(|&c| c == 'C' || c == 'F')
        */
        .collect::<Vec<char>>();

    let mut flg = false;
    let mut ans = false;
    for c in s {
        if c == 'C' { flg = true; }
        if c == 'F' && flg { ans = true; }
    }
    if ans { println!("Yes"); }
    else { println!("No"); }
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
