pub mod uf;
pub mod bsearch;
pub mod graph;

#[allow(dead_code)]
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a }
    else { gcd(b, a % b) }
}

#[allow(dead_code)]
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[allow(dead_code)]
fn divide(n: usize) -> Option<usize> {
    for i in 2..(n as f64).sqrt().ceil() as usize + 1 {
        if n % i == 0 {
            return Some(i);
        }
    }
    return None;
}

#[allow(dead_code)]
/// returns vec of prime factors of n
/// it also contains 1 and n itself
pub fn prime_factor(n: usize) -> Vec<usize> {
    let mut x = n;
    let mut factors = Vec::new();
    while let Some(f) = divide(x) {
        x /= f;
        factors.push(f);
    }
    factors.push(x);
    return factors;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime_factor() {
        let ans: Vec<usize> = vec![2, 2, 3];
        assert_eq!(
            ans,
            prime_factor(12)
        );

        let ans: Vec<usize> = vec![19, 3];
        assert_eq!(
            ans,
            prime_factor(57)
        );
    }
}
