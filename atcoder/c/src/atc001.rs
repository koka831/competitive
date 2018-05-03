#![allow(unused_imports)]
use std::io;
use std::cmp;


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

fn check_bound(x: usize, y: usize, w: usize, h: usize) -> bool {
    return (0 <= x && x < w) && (0 <= y && y < h)
}

type Pos = (usize, usize);

fn dfs(f: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    f[x][y] = 's';

    true
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
