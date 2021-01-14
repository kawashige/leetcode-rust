use rand::{thread_rng, Rng};

pub struct Solution {
    n_rows: i32,
    n_cols: i32,
    flip_targets: Vec<Vec<i32>>,
}

impl Solution {
    fn new(n_rows: i32, n_cols: i32) -> Self {
        Self {
            n_rows,
            n_cols,
            flip_targets: Self::generate_positions(n_rows, n_cols),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let mut rng = thread_rng();
        self.flip_targets
            .remove(rng.gen_range(0, self.flip_targets.len()))
    }

    fn reset(&mut self) {
        self.flip_targets = Self::generate_positions(self.n_rows, self.n_cols);
    }

    fn generate_positions(n_rows: i32, n_cols: i32) -> Vec<Vec<i32>> {
        (0..n_rows)
            .map(|i| (0..n_cols).map(move |j| vec![i, j]))
            .flatten()
            .collect()
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
