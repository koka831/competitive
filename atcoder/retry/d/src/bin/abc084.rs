use std::io;


fn main() {
    let q = read_one::<usize>();
    let mut cnt = vec![0; 100_001];
    for i in 3..100_000 {
        cnt[i] = cnt[i - 1];
        if is_prime(i) && is_prime((i + 1) / 2) {
            cnt[i] += 1;
        }
    }

    for _ in 0..q {
        let (l, r) = {
            let i = read::<usize>();
            (i[0] - 1, i[1])
        };
        println!("{}", cnt[r] - cnt[l]);
    }
}

fn is_prime(n: usize) -> bool {
    if n < 2 { false }
    else if n == 2 { true }
    else {
        (2..(n as f64).sqrt() as usize + 2).all(|i| n % i != 0)
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
