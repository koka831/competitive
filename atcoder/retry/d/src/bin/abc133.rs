use std::io;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();

    let ss = an.iter().sum::<usize>() / 2;
    let mut ans = Vec::new();
    // let mut oo = Vec::new();


    for i in 0..n {
        let mut o: usize = 0;
        for j in 0..(n / 2) {
            o += an[(i + 1 + j * 2) % n];
        }
        ans.push((ss - o) * 2);
    }

    println!("{}", ans.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
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
