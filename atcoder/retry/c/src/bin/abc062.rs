use std::io;
use std::cmp;


fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ans = 1_000_000_000;
    for &(a, b) in [(h, w), (w, h)].iter() {
        for x in 0..b {
            let sa = a * x;
            let b1 = b - x;
            let sb1 = a * (b1 / 2);
            let sc1 = a * (b1 - b1 / 2);
            let smin1 = cmp::min(sa, cmp::min(sb1, sc1));
            let smax1 = cmp::max(sa, cmp::max(sb1, sc1));
            let sb2 = (a / 2) * b1;
            let sc2 = (a - a / 2) * b1;
            let smin2 = cmp::min(sa, cmp::min(sb2, sc2));
            let smax2 = cmp::max(sa, cmp::max(sb2, sc2));
            ans = cmp::min(ans, cmp::min(smax1 - smin1, smax2 - smin2));
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
