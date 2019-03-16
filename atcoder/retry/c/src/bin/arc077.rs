use std::io;
use std::collections::VecDeque;


/// https://beta.atcoder.jp/contests/arc077/tasks/arc077_a
fn main() {
    let n = read_one::<usize>();
    let an = read::<isize>();
    let mut v = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 { v.push_back(an[i]); }
        else { v.push_front(an[i]); }
    }

    let mut v = v.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(); 
    if n % 2 != 0 {
        v.reverse();
        println!("{}", v.join(" "));
    } else {
        println!("{}", v.join(" "));
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
