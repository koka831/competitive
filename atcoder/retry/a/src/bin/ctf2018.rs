use std::io;


fn main() {
    let (t, a, b, c, d) = {
        let i = read::<usize>();
        (i[0], i[1], i[2], i[3], i[4])
    };

    let mut ans = 0;
    if t >= a + c {
        ans = b + d;
    } else if t >= c {
        ans = d;
    } else if t >= a {
        ans = b;
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
