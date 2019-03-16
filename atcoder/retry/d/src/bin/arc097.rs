use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/arc097/tasks/arc097_b
fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let p = read::<usize>();
    let mut uf = UnionFind::new(n + 1);
    for _ in 0..m {
        let i = read::<usize>();
        uf.unite(i[0], i[1]);
    }

    let mut ans = 0;
    for i in 0..p.len() {
        if uf.same(p[i], i + 1) {
            ans += 1;
        }
    }
    println!("{}", ans);
}


#[derive(Debug)]
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x { x }
        else {
            let y = self.par[x];
            self.par[x] = self.find(y);
            self.par[x]
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y { return; }

        match self.rank[x].cmp(&self.rank[y]) {
            cmp::Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
            },
            cmp::Ordering::Less => {
                self.par[x] = y;
            },
            cmp::Ordering::Greater => {
                self.par[y] = x;
            }
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


