use std::io;


/// https://beta.atcoder.jp/contests/abc062/tasks/abc062_b
fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut vec = Vec::new();
    let template = vec!['#'; w + 2];
    vec.push(template.clone());
    for _ in 0..h {
        let mut i = read_one::<String>().chars().collect::<Vec<char>>();
        i.push('#');
        i.insert(0, '#');
        vec.push(i);
    }
    vec.push(template.clone());
    for v in vec {
        for c in v {
            print!("{}", c);
        }
        println!();
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
