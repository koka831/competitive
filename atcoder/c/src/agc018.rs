use std::io;


/// https://beta.atcoder.jp/contests/agc018/tasks/agc018_a
fn main() {
    let (_n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let a = read::<usize>();

    let max = a.iter().max().unwrap();

    let gcd = a.iter()
        .fold(a[0], |x, &y| gcd(x, y));

    if k % gcd == 0 && k <= *max {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }

}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0{
        a
    } else {
        gcd(b, a % b)
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
