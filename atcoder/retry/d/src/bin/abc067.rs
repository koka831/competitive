use std::io;
use std::collections::VecDeque;


fn main() {
    let n = read_one::<usize>();
    let mut t = vec![Vec::new(); n];
    for _ in 1..n {
        let (a, b) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        t[a].push(b);
        t[b].push(a);
    }
    let fen = bfs(0, &t);
    let snu = bfs(n - 1, &t);

    let mut cnt = 0;
    for i in 0..n {
        if fen[i] <= snu[i] { cnt += 1; }
    }
    if cnt > n - cnt { println!("Fennec"); }
    else { println!("Snuke"); }
}

fn bfs(s: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut que = VecDeque::new();
    let mut dist = vec![std::usize::MAX; g.len()];
    dist[s] = 0;
    que.push_back(s);
    while let Some(v) = que.pop_front() {
        for &u in &g[v] {
            if dist[u] <= dist[v] { continue; }
            dist[u] = dist[v] + 1;
            que.push_back(u);
        }
    }
    dist
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
