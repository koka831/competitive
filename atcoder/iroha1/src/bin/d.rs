use std::io;


fn main() {
    let (n, x, y) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut an = read::<usize>();
    an.sort(); an.reverse();
    let mut tak = x;
    let mut aok = y;
    for i in 0..n {
        if i % 2 == 0 { tak += an[i]; }
        else { aok += an[i]; }
    }
    if tak > aok { println!("Takahashi"); }
    else if aok > tak { println!("Aoki"); }
    else { println!("Draw"); }
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
