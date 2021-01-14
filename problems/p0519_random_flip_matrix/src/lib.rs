use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub struct Solution {
    n_rows: i32,
    n_cols: i32,
    flipped: HashSet<(i32, i32)>,
}

impl Solution {
    fn new(n_rows: i32, n_cols: i32) -> Self {
        Self {
            n_rows,
            n_cols,
            flipped: Default::default(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut n = rng.gen_range(0, self.n_rows * self.n_cols);
        let mut row = n / self.n_cols;
        let mut column = n % self.n_cols;
        while self.flipped.contains(&(row, column)) {
            n = rng.gen_range(0, self.n_rows * self.n_cols);
            row = n / self.n_cols;
            column = n % self.n_cols;
        }
        self.flipped.insert((row, column));
        vec![row, column]
    }

    fn reset(&mut self) {
        self.flipped.clear();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0519() {
        let mut obj = Solution::new(2, 3);
        let flip = obj.flip();
        assert!(0 <= flip[0] && flip[0] < 2 && 0 <= flip[1] && flip[1] < 3);
    }
}
