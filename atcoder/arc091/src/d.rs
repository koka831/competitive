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

    let (n, k) = {
        let val = read::<isize>();
        (val[0], val[1])
    };

    /*
    let mut cnt = (n - k) * (n - k + 1) / 2;

    'outer: for i in (k * 2)..(n + 1) {
        'inner: for j in 1..i {
            if i % j >= k {
                cnt += 1;
            }
        }
    }
    */

    // after saw the editorial
    let mut cnt = 0;
    for b in 1..(n + 1) {
        let p = n / b;
        let q = n % b;
        cnt += p * cmp::max(0, b - k) + cmp::max(0, q - k + 1);

        if k == 0 { cnt -= 1; }
    }

    println!("{}", cnt);
}
