use std::io;
use std::cmp;


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

    let (n, m) = {
        let val = read::<isize>();
        (val[0], val[1])
    };

    if n == 2 || m == 2 {
        println!("0");
    } else {
        let x = cmp::max(1, n - 2);
        let y = cmp::max(1, m - 2);
        println!("{}", x * y);
    }
}
