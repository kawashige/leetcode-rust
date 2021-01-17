use rand::{thread_rng, Rng};

pub struct Solution {
    weight_sums: Vec<i32>,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut weight_sums = vec![w[0]];
        for i in 1..w.len() {
            weight_sums.push(w[i] + weight_sums[i - 1]);
        }
        Self { weight_sums }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = thread_rng();
        match self
            .weight_sums
            .binary_search(&rng.gen_range(1, self.weight_sums.last().unwrap() + 1))
        {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0528() {
        let mut obj = Solution::new(vec![1, 3]);
        let n1 = obj.pick_index();
        println!("n1: {}", n1);
        assert!(0 <= n1 && n1 < 2);
    }
}
