use std::io;
use std::cmp;


fn main() {
    let (x, y) = {
        let i = read::<isize>();
        (i[0], i[1])
    };

    let mut ans = 10_000_000_000_000;

    for s in 0..2 { for t in 0..2 {
        let mut cnt = 0;
        let mut _x = x;
        let mut _y = y;
        if s > 0 { _x = -x; cnt += 1; }
        if t > 0 { _y = -y; cnt += 1; }
        if _x <= _y {
            cnt += _y - _x;
        } else { continue; }

        ans = cmp::min(ans, cnt);
    }}

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
