use std::io;


fn main() {
    let (w, a, b) = {
        let i = read::<isize>();
        (i[0], i[1], i[2])
    };

    if (a - b).abs() <= w { println!("0"); }
    else { println!("{}", (a - b).abs() - w); }
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
