use std::io;


fn main() {
    let (n, m, a, b) = {
        let i = read::<usize>();
        (i[0], i[1], i[2], i[3])
    };

    let mut ans = vec![b; n];
    for _ in 0..m {
        let (l, r) = {
            let i = read::<usize>();
            (i[0], i[1])
        };

        for i in (l - 1)..r {
            ans[i] = a;
        }
    }

    println!("{}", ans.iter().sum::<usize>());
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
