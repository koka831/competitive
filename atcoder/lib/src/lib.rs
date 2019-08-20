use std::cmp;

#[allow(unused)]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[allow(unused)]
fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

#[allow(dead_code)]
fn ncr(n: usize, r: usize) -> usize {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => ncr(n, r - 1) * (n - r + 1) / r,
    }
}


#[allow(unused)]
macro_rules! debug {
    ($($v:expr),*) => { {
        use ::std::io::Write;
        $(let _ = writeln!(::std::io::stderr(), "{} = {:?}", stringify!($v), $v);)*
    } }
}

#[allow(unused)]
static MOD : i64 = (1e9 as i64) + 7;



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
