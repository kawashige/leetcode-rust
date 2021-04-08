use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        answers
            .into_iter()
            .fold(HashMap::new(), |mut counts, a| {
                *counts.entry(a).or_insert(0) += 1;
                counts
            })
            .into_iter()
            .map(|(k, v)| (k + 1) * ((v + k) / (k + 1)))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0781() {
        assert_eq!(Solution::num_rabbits(vec![1, 1, 1]), 4);
        assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
        assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
        assert_eq!(Solution::num_rabbits(vec![]), 0);
    }
}
