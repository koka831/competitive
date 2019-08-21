#[allow(dead_code)]
/// verified: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3822184#2
pub struct SegmentTree<T, F> {
    n: usize,
    v: Vec<T>,
    id: T,
    f: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, id: T, f: F) -> Self {
        SegmentTree {
            n: n,
            v: vec![id.clone(); 2 * n],
            id: id,
            f: f,
        }
    }

    pub fn from(v: &[T], id: T, f: F) -> Self {
        let n = v.len();
        let mut vs = vec![id.clone(); n];
        vs.extend_from_slice(v);
        for i in (1..n).rev() {
            vs[i] = f(&vs[i * 2], &vs[i * 2 + 1]);
        }
        SegmentTree {
            n: n,
            v: vs,
            id: id,
            f: f,
        }
    }

    pub fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while { i /= 2; i > 0 } {
            self.v[i] = (self.f)(&self.v[i * 2], &self.v[i * 2 + 1]);
        }
    }

    pub fn apply(&mut self, s: usize, t: usize) -> T {
        let (mut x, mut y) = (self.id.clone(), self.id.clone());
        let (mut s, mut t) = (s + self.n, t + self.n);
        while s < t {
            if s % 2 != 0 {
                x = (self.f)(&x, &self.v[s]);
                s += 1;
            }
            if t % 2 != 0 {
                t -= 1;
                y = (self.f)(&self.v[t], &y);
            }
            s /= 2;
            t /= 2;
        }
        (self.f)(&x, &y)
    }
}
