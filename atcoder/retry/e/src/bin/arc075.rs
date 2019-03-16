use std::io;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1] as isize)
    };

    let mut an = Vec::new();
    for _ in 0..n { an.push(read_one::<isize>() - k); }

    // cumsum
    let mut bn = vec![0; n];
    bn[0] = an[0];
    for i in 1..n { bn[i] = bn[i - 1] + an[i]; }

    // compress
    let mut bn_s = bn.clone();
    bn_s.sort();
    bn_s.dedup();

    let cn = bn.iter()
        .map(|b| bn_s.binary_search(b).unwrap())
        .collect::<Vec<usize>>();

    let mut bit = FenwickTree::new(bn_s.len());
    // append {single} >= K elements
    let mut ans = bn.into_iter().filter(|&b| b >= 0).count();

    for c in cn {
        ans += bit.sum(c + 1);
        bit.add(c, 1);
    }

    println!("{}", ans);
}


#[derive(Debug)]
struct FenwickTree {
    n: usize,
    bit: Vec<usize>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree { n: n, bit: vec![0; n] }
    }

    // [0, k)
    fn sum(&self, i: usize) -> usize {
        let mut s = 0;
        let mut i = i as isize;
        while i > 0 {
            i -= 1;
            s += self.bit[i as usize];
            i &= i + 1;
        }
        s
    }

    fn add(&mut self, i: usize, x: usize) {
        let mut i = i;
        while i < self.n {
            self.bit[i] += x;
            i |= i + 1;
        }
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
