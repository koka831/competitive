use std::io;


fn main() {
    let (a, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ans: usize = 0;
    let tag: usize = 2 * 10usize.pow(12);
    if k == 0 { ans = tag - a; }
    else {
        let mut t: usize = a;
        loop {
            if t >= tag { break; }
            else {
                t += 1 + k * t;
                ans += 1;
            }
        }
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
