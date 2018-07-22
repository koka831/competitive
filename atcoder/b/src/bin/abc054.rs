use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut a = Vec::new();
    let mut b = Vec::new();

    for _ in 0..n {
        a.push(read_one::<String>().chars().collect::<Vec<char>>());
    }
    for _ in 0..m {
        b.push(read_one::<String>().chars().collect::<Vec<char>>());
    }

    let mut flg = false;
    println!("default a: {:?}", a);
    for i in 0..(n - m) { for j in 0..(n - m) {
            println!("{:?}", &a[i..(m + i)][j..(m + i)]);
        if a[i..m][j..m] == b[..] {
            flg = true;
        }
    }}
    if flg { println!("Yes"); }
    else { println!("No"); }
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
