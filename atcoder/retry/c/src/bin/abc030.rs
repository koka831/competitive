use std::io;
use std::cmp;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let (x, y) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();
    let bm = read::<usize>();
    let mut cnt: usize = 0;
    let mut cur = an[0];
    loop {
        // toB
        let idx_b = bm.lower_bound(&(cur + x));
        if idx_b >= m { break; }
        cnt += 1;
        // toA
        let b = bm[idx_b] + y;
        let idx_a = an.lower_bound(&b);
        if idx_a >= n { break; }
        cnt += 1;
        cur = an[idx_a];
    }

    println!("{}", (cnt + 1) / 2);
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Less => {
                    low = mid + 1;
                },
                _ => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Greater => {
                    high = mid;
                },
                _ => {
                    low = mid + 1;
                }
            }
        }
        low
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
