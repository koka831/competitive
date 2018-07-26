use std::io;


fn main() {
    let (x, y) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    /*
    let ans = ((y / x) as f32).log2();
    println!("{}", ans.floor() as usize + 1);
    */
    let mut ans = 0;
    let mut a = x;
    while a <= y {
        a *= 2;
        ans += 1;
    }
    println!("{}", ans);
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
