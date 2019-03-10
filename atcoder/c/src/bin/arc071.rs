use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut cnt = vec![::std::usize::MAX; 26];

    for _ in 0..n {
        let s_ni = read_one::<String>().chars()
            .collect::<Vec<char>>();
        let mut _cnt = vec![0; 26];
        for s in s_ni {
            _cnt[(s as u8 - 'a' as u8) as usize] += 1;
        }

        for c in 0..26 {
            cnt[c] = cmp::min(cnt[c], _cnt[c]);
        }
    }

    for c in 0..26 {
        for _ in 0..cnt[c] {
            print!("{}", (c + 'a' as usize) as u8 as char);
        }
    }
    println!();
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
