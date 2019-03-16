use std::io;


/// https://beta.atcoder.jp/contests/code-festival-2016-quala/tasks/codefestival_2016_qualB_b
fn main() {
    let (n, a, b) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let sn = read_one::<String>().chars().collect::<Vec<char>>();
    let mut cnt = 0;
    let mut f = 0;
    for i in 0..n {
        match sn[i] {
            'a' => {
                if cnt < a + b { println!("Yes"); cnt += 1; }
                else { println!("No"); }
            },
            'b' => {
                if cnt < a + b && f < b { println!("Yes"); cnt += 1; f += 1; }
                else { println!("No"); }
            },
            'c' => {
                println!("No");
            },
            _ => unreachable!(),
        }
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
