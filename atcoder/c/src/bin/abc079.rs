use std::io;


fn main() {
    let abcd = read_one::<String>().chars()
        .map(|c| c as isize - '0' as isize)
        .collect::<Vec<isize>>();

    // 0: +, 1: -
    for j in 0..1 << (abcd.len() - 1) {
        let mut sum = abcd[0];
        let mut ans = abcd[0].to_string();
        let mut vec = Vec::new();
        for k in 0..(abcd.len() - 1) {
            vec.push(j & (1 << k));
            if (j & 1 << k) != 0 {
                sum -= abcd[k + 1];
                ans.push_str(format!("-{}", abcd[k + 1]).as_str());
            } else {
                sum += abcd[k + 1];
                ans.push_str(format!("+{}", abcd[k + 1]).as_str());
            }
        }
        if sum == 7 {
            println!("{}=7", ans);
            break;
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
