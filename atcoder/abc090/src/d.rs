use std::io;
use std::cmp;

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

fn main() {

    let (n, k) = {
        let a = read::<isize>();
        (a[0], a[1])
    };

    let mut cnt = 0;
    for b in 1..(n + 1) {
        let p = n / b;
        let q = n % b;
        cnt += p * cmp::max(0, b - k) + cmp::max(0, q - k + 1);
        if k == 0 { cnt -= 1; }
    }

    println!("{}", cnt);
}
