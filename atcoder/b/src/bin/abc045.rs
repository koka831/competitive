use std::io;


fn main() {
    let mut a = read_one::<String>().chars().rev().collect::<Vec<char>>();
    let mut b = read_one::<String>().chars().rev().collect::<Vec<char>>();
    let mut c = read_one::<String>().chars().rev().collect::<Vec<char>>();

    let mut past = a.pop().unwrap();
    loop {
        match past {
            'a' => {
                if a.len() > 0 {
                    past = a.pop().unwrap();
                } else { println!("A"); break; }
            },
            'b' => {
                if b.len() > 0 {
                    past = b.pop().unwrap();
                } else { println!("B"); break; }
            },
            'c' => {
                if c.len() > 0 {
                    past = c.pop().unwrap();
                } else { println!("C"); break; }
            },
            _ => unreachable!()
        }
    }
}

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
