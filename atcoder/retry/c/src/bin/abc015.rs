use std::io;


fn main() {
    let (n, _) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut tn = Vec::new();
    for _ in 0..n { tn.push(read::<usize>()); }
    let ans = dfs(n, 0, &tn);

    if ans { println!("Found"); }
    else { println!("Nothing"); }
}

fn dfs(n: usize, acc: usize, tn: &[Vec<usize>]) -> bool {
    if n == 0 { acc == 0 }
    else {
        tn[n - 1].iter()
            .map(|&t| dfs(n - 1, acc ^ t, tn))
            .any(|b| b)
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
