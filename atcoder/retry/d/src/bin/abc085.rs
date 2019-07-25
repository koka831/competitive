use std::io;


fn main() {
    let (n, h) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ab = Vec::new();
    for _ in 0..n {
        let (a, b) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        ab.push((a, b));
    }

    ab.sort_by_key(|x| x.0);
    let a_max = ab[ab.len() - 1].0;
    let b = ab.into_iter()
        .filter(|x| x.1 >= a_max)
        .collect::<Vec<(usize, usize)>>();
    let b_len = b.len();
    let b_sum = b.iter().map(|x| x.1).sum::<usize>();

    println!("{}", b_len + (h - b_sum) / a_max);
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
