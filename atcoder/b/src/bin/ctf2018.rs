use std::io;


fn main() {
    let (x, y) = {
        let i = read::<isize>();
        (i[0], i[1])
    };
    let mut ans = true;
    if (x + y) % 4 != 0 { ans = false; }

    let cnt = (x + y) / 4;
    if x < cnt || y < cnt { ans = false; }
    if (x - cnt) % 2 != 0 || (y - cnt) % 2 != 0 { ans = false; }

    if ans {
        println!("Yes");
    } else {
        println!("No");
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
