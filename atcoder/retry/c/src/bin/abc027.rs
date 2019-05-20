use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut x = 1;
    let mut cnt = 0;
    let d = (n as f64).log2() as usize;

    while x <= n {
        if d % 2 == 0 {
            if cnt % 2 == 0 { x = x * 2 + 1; }
            else { x *= 2; }
        } else {
            if cnt % 2 == 0 { x *= 2; }
            else { x = x * 2 + 1; }
        }

        if x > n {
            if cnt % 2 != 0 { println!("Takahashi"); }
            else { println!("Aoki"); }
            break;
        }
        cnt += 1;
    }
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
