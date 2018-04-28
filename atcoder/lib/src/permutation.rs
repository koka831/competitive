// from github bluss/permutohedron

pub trait LexicalPermutation {
    fn next_permutation(&mut self) -> bool;
    fn prev_permutation(&mut self) -> bool;
}

impl<T: Ord> LexicalPermutation for [T] {

    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 { return false; }

        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] { i -= 1; }

        if i == 0 { return false; }

        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] { j -= 1; }
        self.swap(j, i - 1);

        self[i..].reverse();
        true
    }

    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 { return false; }

        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] { i -= 1; }

        if i == 0 { return false; }
        while j >= i && self[j - 1] < self[i - 1] { j -= 1; }

        self.swap(i - 1, j);
        true
    }
}