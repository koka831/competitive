use std::io;
use std::collections::HashMap;


fn main() {
    let (n, k, l) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let mut uf_road = UnionFind::new(n);
    let mut uf_train = UnionFind::new(n);

    for _ in 0..k {
        let (p, q) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        uf_road.unite(p, q);
    }

    for _ in 0..l {
        let (r, s) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        uf_train.unite(r, s);
    }

    let mut hm = HashMap::new();

    for i in 0..n {
        let key = (uf_road.find(i), uf_train.find(i));
        *hm.entry(key).or_insert(0) += 1;
    }

    let mut ans = Vec::new();
    for i in 0..n {
        let key = (uf_road.find(i), uf_train.find(i));
        let cnt: usize = *hm.get(&key).unwrap_or(&0);
        ans.push(cnt.to_string());
    }

    println!("{}", ans.join(" "));
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn unite(&mut self, a: usize, b: usize) {
        use std::cmp;
        let x = self.find(a);
        let y = self.find(b);

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
