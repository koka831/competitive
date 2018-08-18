use std::io;


fn main() {
    let (x, y) = {
        let i = read::<isize>();
        (i[0], i[1])
    };

    let gx = match x {
        1|3|5|7|8|10|12 => 1,
        4|6|9|11 => 2,
        2 => 3,
        _ => 4,
    };

    let gy = match y {
        1|3|5|7|8|10|12 => 1,
        4|6|9|11 => 2,
        2 => 3,
        _ => 4,
    };

    if gx == gy { println!("Yes"); }
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
