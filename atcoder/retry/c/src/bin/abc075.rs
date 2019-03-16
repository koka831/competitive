use std::io;
use std::cmp;
use std::collections::HashMap;


/// https://beta.atcoder.jp/contests/abc075/tasks/abc075_c
fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut g = Vec::new();
    for _ in 0..m {
        let i = read::<usize>();
        g.push((i[0] - 1, i[1] - 1));
    }

    let mut cnt = 0;

    for j in 0..m {
        let mut  g_clone = g.clone();
        g_clone.remove(j);

        let mut uf = UnionFind::new(n);
        let mut edge = vec![0; n];

        for (u, v) in g_clone {
            uf.unite(u, v);
            edge[u] += 1;
            edge[v] += 1;
        }

        let mut hm = HashMap::new();

        for k in 0..n {
            hm.entry(uf.find(k))
                .or_insert(Vec::new()).push(k);
        }

        // println!("{:?}", hm);
        // println!("{:?}", edge);
        if hm.len() > 1 { cnt += 1; }
    }

    println!("{}", cnt);
}


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
            let p = self.par[x];
            let pp = self.find(p);
            self.par[x] == pp;
            pp
        }
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
