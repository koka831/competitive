use std::io;


/// https://beta.atcoder.jp/contests/arc081/tasks/arc081_a
fn main() {
    let _n = read_one::<usize>();
    let mut a = read::<usize>();
    a.sort_by(|a, b| b.cmp(a));

    let mut vec = Vec::new();
    for a_t in a.iter() {
        let cnt = a.iter().filter(|&x| x == a_t).count();
        if cnt >= 4 { vec.push(a_t); vec.push(a_t); }
        else if cnt >= 2 { vec.push(a_t); }
    }

    println!("{:?}", vec);
    println!("{:?}", a);
    if vec.len() >= 4 {
        vec.sort_by(|a, b| b.cmp(a));
        println!("{}", vec[0] * vec[2]);
    } else {
        println!("0");
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
