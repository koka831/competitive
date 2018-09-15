use std::io;

fn main() {
    let (n, mut x) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut an = read::<usize>();
    an.sort();
    let mut ans = 0;

    for i in 0..n {
        if i == n - 1 {
            if an[i] == x { ans += 1; }
        } else if an[i] <= x { x -= an[i]; ans += 1; }
        else { break; }
    }
    println!("{}", ans);
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
