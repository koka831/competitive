use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();

    let mut node = vec![Vec::new(); n];
    for i in 1..n {
        let x = read_one::<usize>() - 1;
        node[x].push(i);
    }

    let mut score = vec![0; n];
    for i in (0..n).rev() {
        score[i] = if node[i].len() == 0 { 1 }
        else {
            let mut max = 0;
            let mut min = 100_000_000;
            for &v in node[i].iter() {
                max = cmp::max(max, score[v]);
                min = cmp::min(min, score[v]);
            }
            // println!("user{}: node: {:?}, max: {}, min: {}", i, node[i], max, min);
            max + min + 1
        }
    }
    println!("{}", score[0]);
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
