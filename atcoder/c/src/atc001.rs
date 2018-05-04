#![allow(unused)]
use std::io;
use std::cmp;

const INF: usize = 100_000_000_000;


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

fn check_bound(x: isize, y: isize, w: usize, h: usize) -> bool {
    return (0 <= x && x < w as isize) && (0 <= y && y < h as isize)
}

type Pos = (usize, usize);

/*
fn dfs(f: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    f[x][y] = 's';
    let n = f.len();
    let mut dist = vec![n + 1; n];
    let mut queue = ::std::collections::VecDeque::new();
    dist[] = 0;
    queue.push_back(s);
    true
}
*/

fn bfs(g: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let n = g.len();
    let mut dist = vec![n + 1; n];
    let mut queue = ::std::collections::VecDeque::new();
    dist[s] = 0;
    queue.push_back(s);
    while let Some(u) = queue.pop_front() {
        for &v in &g[u] {
            if dist[v] == n + 1 {
                dist[v] = dist[u] + 1;
                // if s == '#' { /* return true */}
                queue.push_back(v);
            }
        }
    }
    dist
}

fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vec = Vec::new();
    for _ in 0..h {
        vec.push(read::<char>());
    }

    for j in 0..h { for k in 0..w {
        println!("{} {}", j, k);
    }}

    /*
    if dfs(&vec, 0, 0) { 
        println!("Yes"); 
    } else { 
        println!("No");
    }
    */
}
