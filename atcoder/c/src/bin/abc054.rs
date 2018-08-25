use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut am = (0..n).collect::<Vec<usize>>();
    let mut path = vec![vec![false; n]; n];
    for _ in 0..m {
        let i = read::<usize>();
        let (a, b) = (i[0] - 1, i[1] - 1);
        path[a][b] = true;
        path[b][a] = true;
    }

    let mut ans = 0;
    loop {
        if am[0] == 0 { // needs scope to borrow as mutable
            let mut one = am.windows(2)
                .map(|e| path[e[0]][e[1]]);
            if one.all(|flg| flg) { ans += 1; }
        }
        if !next_permutation(&mut am) { break; }
    }
    println!("{}", ans);
}


#[allow(dead_code)]
/// https://github.com/bluss/permutohedron/blob/master/src/lexical.rs
fn next_permutation<T>(list: &mut [T]) -> bool
where T:
    ::std::cmp::PartialOrd
{
    let mut i = list.len() - 1;
    while i > 0 && list[i - 1] >= list[i] { i -= 1; }
    if i <= 0 { return false; }

    let mut j = list.len() - 1;
    while list[j] <= list[i - 1] { j -= 1; }

    list.swap(i - 1, j);
    list[i..].reverse();
    true
}



#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
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
