use std::io;
use std::collections::HashSet;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        let (x, y, _) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1, i[2])
        };
        uf.unite(x, y)
    }

    let mut hs = HashSet::new();
    for i in 0..n { hs.insert(uf.find(i)); }
    println!("{}", hs.len());
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {

    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0;n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] { x }
        else {
            let par = self.par[x];
            self.par[x] = self.find(par);
            self.par[x]
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        use std::cmp::Ordering;
        let x = self.find(x);
        let y = self.find(y);
        if x == y { return; }

        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
            },
            Ordering::Less => {
                self.par[x] = y;
            },
            Ordering::Greater => {
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
