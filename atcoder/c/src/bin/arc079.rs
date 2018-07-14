use std::io;


/// https://beta.atcoder.jp/contests/arc079/tasks/arc079_a
fn main() {

    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut from = vec![false; n + 1];
    let mut to = vec![false; n + 1];
    let mut path = Vec::new();

    for _ in 0..m {
        let (s, t) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        if s == 1 { from[t] = true; }
        if t == n { to[s] = true; }
        path.push(s);
        path.push(t);
    }

    let mut ans = false;
    for j in path.into_iter() {
        if from[j] && to[j] {
            ans = true;
        }
    }

    if ans { println!("POSSIBLE"); }
    else { println!("IMPOSSIBLE"); }

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
