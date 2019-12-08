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
