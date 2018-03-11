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

    let (n, k) = {
        let val = read::<usize>();
        (val[0], val[1])
    };

    let mut cnt = (n - k) * (n - k + 1) / 2;

    'outer: for i in (k * 2)..(n + 1) {
        'inner: for j in 1..i {
            if i % j >= k {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
