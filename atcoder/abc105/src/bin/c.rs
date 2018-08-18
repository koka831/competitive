use std::io;


fn main() {
    // let n = read_one::<String>().chars().collect::<Vec<char>>();
    let n = read_one::<isize>();
    if n > 0 {
        let bin = format!("{:#b}", n);
        let mut buf = vec![0; bin.len() + 1];
        for (i, b) in bin.chars().enumerate() {
            if i % 2 == 0 {
                if b == '1' { buf[i] = 1; }
            } else {
                if b == '1' { buf[i+1] = 1; buf[i] = 1; }
            }
        }
        let ans = buf.iter().map(|&c| c.to_string()).collect::<String>();
        let mut l = 0;
        let mut r = buf.len() - 1;
        loop {
            if buf[l] == 0 { l += 1; } else { break; }
        }
        loop {
            if buf[r] == 0 { r -= 1; } else { break; }
        }
        let a = &ans[l..r + 1];
        println!("{:}", a);

    } else if n == 0 {
        println!("0");
    } else {
        let bin = format!("{:#b}", 0 - n);
        let mut buf = vec![0; bin.len() + 1];
        for (i, b) in bin.chars().enumerate() {
            if i % 2 != 0 {
                if b == '1' { buf[i] = 1; }
            } else {
                if b == '1' { buf[i+1] = 1; buf[i] = 1; }
            }
        }
        let ans = buf.iter().map(|&c| c.to_string()).collect::<String>();
        let mut l = 0;
        let mut r = buf.len() - 1;
        loop {
            if buf[l] == 0 { l += 1; } else { break; }
        }
        loop {
            if buf[r] == 0 { r -= 1; } else { break; }
        }
        let a = &ans[l..r + 1];
        println!("{:}", a);
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
