use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/bitflyer2018-qual/tasks/bitflyer2018_qual_b
fn main() {
    let (a, b, _) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let x = read_one::<String>().chars().collect::<Vec<char>>();
    let mut short = a as isize;
    let mut cheese = b as isize;

    for c in x {
        if c == 'S' { short -= 1; }
        if c == 'C' { cheese -= 1; }
        if c == 'E' {
            if short == cheese { short -= 1; }
            else if short < cheese { cheese -= 1; }
            else { short -= 1; }
        }
    }

    println!("{}", cmp::max(0, short));
    println!("{}", cmp::max(0, cheese));
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
