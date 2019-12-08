use std::io;
use std::cmp;


fn main() {
    let (x, y) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    // n + 2m = x, 2n + m = y
    // m = (2x - y) / 3
    // n = (2y - x) / 3
    if (x + y) % 3 != 0 { println!("0"); return; }
    let m = Modulo::new(1_000_000, 1_000_000_007);
    println!("{}", m.comb((x + y) / 3, (2 * cmp::max(x, y) - cmp::min(x, y)) / 3));
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
        if n < r { return 0; }
        self.fact[n] * self.inv_fact[r] %
            self.modulo * self.inv_fact[n - r] % self.modulo
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
