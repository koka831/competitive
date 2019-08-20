use std::io;
use std::collections::HashMap;

const MOD: usize = (1e9 as usize) + 7;

fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ans: usize = 1;
    let modulo = Modulo::new(1_000_000, MOD);
    let pf = prime_factor(m);
    // println!("{:?}", pf);
    for v in pf.values() {
        // n H r = m C r, where m = n + r - 1
        ans *= modulo.comb(n + v - 1, *v);
        ans %= MOD;
    }
    println!("{}", ans);
}

pub struct Modulo {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize
}

impl Modulo {
    pub fn new(n: usize, modulo: usize) -> Self {
        let mut fact = vec![0; n + 1];
        let mut inv = vec![0; n + 1];
        let mut inv_fact = vec![0; n + 1];
        inv[1] = 1;
        for i in 2..n + 1 {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }
        fact[0] = 1;
        inv_fact[0] = 1;
        for i in 0..n {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }
        for i in 0..n {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Modulo { fact: fact, inv_fact: inv_fact, modulo: modulo }
    }

    pub fn comb(&self, n: usize, r: usize) -> usize {
        self.fact[n] * self.inv_fact[r] %
            self.modulo * self.inv_fact[n - r] % self.modulo
    }
}

fn divide(n: usize) -> Option<usize> {
    for i in 2..(n as f64).sqrt().ceil() as usize + 1 {
        if n % i == 0 { return Some(i); }
    }
    None
}

pub fn prime_factor(n: usize) -> HashMap<usize, usize> {
    let mut x = n;
    let mut hm = HashMap::new();
    while let Some(f) = divide(x) {
        x /= f;
        *hm.entry(f).or_insert(0) += 1;
    }
    if x != 1 {
        *hm.entry(x).or_insert(0) += 1;
    }
    hm
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
