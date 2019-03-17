use std::collections::VecDeque;

fn dfs(g: &mut Vec<Vec<usize>>, x: usize) -> Vec<usize> {
    let n = g.len();
    let mut dist = vec![x; n];
    let mut q = VecDeque::new();
    dist[x] = 0;
    q.push_back(x);
    while let Some(u) = q.pop_front() {
        for &v in &g[u] {
            if dist[v] == n + 1 {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }
    dist
}

struct Grid {
    g: Vec<Vec<usize>>,
    W: usize,
    H: usize,
}

type Pos = (usize, usize);

impl Grid {
    fn check_bound(&self, xy: Pos) -> bool {
        (0 <= xy.0 && xy.0 < self.W) && (0 <= xy.1 && xy.1 < self.H)
    }
}
