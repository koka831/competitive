use std::io;


fn main() {
    let _s = read_one::<String>();
    let _t = read_one::<String>();
    let s = if _s.len() > _t.len() { _s.clone() } else { _t.clone() };
    let t = if _s.len() < _t.len() { _s } else { _t };
    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();

    let mut idx = 0;
    let mut size = 0;
    for i in 0..t.len() {
        let mut _size = 0;
        let mut _str = "".to_string();
        for j in 0..s.len() {
            if s[i + j] == t[i + j] {
                _size += 1;
            }
            else { break; }
        }
        if _size > size {
            idx = i;
            size = _size;
        }
    }

    for i in idx..(idx + size + 1) {
        print!("{}", s[i]);
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
