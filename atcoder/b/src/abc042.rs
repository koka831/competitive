use std::io;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
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

fn main() {
    let (n, _l) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vec = Vec::new();

    for _ in 0..n {
        vec.push(read_one::<String>());
    }

    vec.sort();
    let ans = vec.into_iter()
        .fold(String::from(""), |a, b| a + &b);

    println!("{}", ans);
}
