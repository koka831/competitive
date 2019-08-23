use std::io;


fn main() {
    let (w, h, x, y) = {
        let i = read::<f64>();
        (i[0], i[1], i[2], i[3])
    };

    let area = w * h / 2.0;
    let way = if x + x == w && y + y == h { 1 } else { 0 };
    println!("{} {}", area, way);
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
