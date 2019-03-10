use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ab = Vec::new();
    for _ in 0..m {
        let (a, b) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        ab.push((a, b));
    }

    let mut uf = UnionFind::new(n + 1);
    let mut cost = n * (n - 1) / 2;
    let mut ans = Vec::new();

    for (a, b) in ab.into_iter().rev() {
        ans.push(cost);
        if uf.same(a, b) { continue; }
        let pa = uf.find(a);
        let pb = uf.find(b);
        let c = uf.siz[pa] * uf.siz[pb];
        uf.unite(a, b);
        cost -= c;
    }

    for a in ans.iter().rev() {
        println!("{}", a);
    }
}


struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn unite(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb { return; }
        self.siz[pa] += self.siz[pb];
        self.par[pa] = pb;
        self.siz[pb] = self.siz[pa];
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn find(&mut self, a: usize) -> usize {
        if self.par[a] == a { a }
        else {
            let p = self.par[a];
            self.par[a] = self.find(p);
            self.par[a]
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
