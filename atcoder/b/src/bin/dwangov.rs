use std::io;


fn main() {
    let n = read_one::<f64>();
    let an = read::<f64>();
    let avg = an.iter().sum::<f64>() / n;
    /* sort for PartialOrd...
    let mut ans = an.iter().enumerate()
        .map(|(i, a)| (i, (a - avg).abs()))
        .collect::<Vec<(usize, f64)>>();
    ans.sort_by_key(|a| a.1);
    */
    let mut id = 0;
    let mut score = 1000.0;
    for (i, a) in an.iter().enumerate() {
        let s = (a - avg).abs();
        if s < score {
            id = i;
            score = s;
        }
    }
    println!("{:?}", id);
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
