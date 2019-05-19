use std::io;


fn main() {
    let n = read_one::<usize>();

    for i in 1..n + 1 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(n: usize) -> String {
    match(n % 3, n % 5) {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        _ => n.to_string()
    }
}

fn binary_search(i: usize) -> usize {
    let mut left = 0;
    let mut right = i;

    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if (/* cond */true) { right = mid; }
        else { left = mid; }
    }
    // left: 条件を満たさない最大
    // right: 条件を満たす最小
    right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_1() {
        assert_eq!(fizzbuzz(1), "1".to_string());
    }

    #[test]
    fn fizzbuzz_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz".to_string());
    }

    #[test]
    fn fizzbuzz_buzz() {
        assert_eq!(fizzbuzz(5), "Buzz".to_string());
    }

    #[test]
    fn fizzbuzz_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz".to_string());
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
