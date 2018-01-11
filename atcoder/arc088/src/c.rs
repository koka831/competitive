use std::io;


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


fn main() {
    let (mut x, y) = {
        let v = read::<i64>();
        (v[0], v[1])
    };

    let mut cnt = 0;
    while x <= y {
        x *= 2;
        cnt += 1;
    }

    println!("{}", cnt);
}
