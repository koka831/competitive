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
    fn sum(&self, k: usize) -> usize {
        let mut s = 0;
        let mut i = k;
        while i > 0 {
            i -= 1;
            s += self.bit[i];
            i &= i + 1;
        }
        s
    }

    fn add(&mut self, i: usize, v: usize) {
        let mut i = i;
        while i < self.n {
            self.bit[i] += v;
            i |= i + 1;
        }
    }
}
