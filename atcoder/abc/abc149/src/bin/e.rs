use std::io;
use std::cmp;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut an = read::<usize>();
    an.sort();
    an.reverse();
    let mut cnt: usize = 0;
    let mut l = 0;
    let mut r = *an.iter().max().unwrap() * 2;
    while r - l > 1 {
        let mid = (l + r) / 2;
        let c = an.iter().map(|&a| {
            // count x s.t. x + a >= m;
            an.upper_bound(cmp::max(0, mid as isize - a as isize) as usize)
        }).sum();
        if c >= m {
            l = mid;
        } else {
            r = mid;
            cnt = c;
        }
    }
    let mut ans = (m - cnt) * r;
    println!("{}:{}", cnt, r);
    // rより大きいpairをcnt件作成する
    println!("{}", ans);
}

trait BinarySearch<T> {
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn upper_bound(&self, x: T) -> usize {
        let mut l = -1isize;
        let mut r: isize = (self.len() - 1) as isize;

        while (r - l) as usize > 1 {
            let m = (l + r) / 2;
            if self[m as usize] > x { l = m; }
            else { r = m; }
        }
        (l + 1isize) as usize
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
