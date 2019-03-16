use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let an = read::<i64>();
    let mut ans = ::std::usize::MAX;

    for j in 0..2 {
        let mut step: i64 = 0;
        let mut cumsum = 0;
        for i in 0..n {
            cumsum += an[i];
            if (i + j) % 2 == 0 { // minus
                if cumsum >= 0 {
                    step += cumsum + 1;
                    cumsum = -1;
                }
            } else { // plus
                if cumsum <= 0 {
                    step += 1 - cumsum;
                    cumsum = 1;
                }
            }
        }
        ans = cmp::min(ans, step as usize);
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
