pub struct Solution {}

impl Solution {
    pub fn recurse(i: usize, cost: &[i32]) -> (i32, i32) {
        if cost.len() < i {
            return (0, 0);
        }
        let left = Self::recurse(i * 2, cost);
        let right = Self::recurse(i * 2 + 1, cost);
        (
            cost[i - 1] + left.0.max(right.0),
            left.1 + right.1 + (left.0 - right.0).abs(),
        )
    }
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        Self::recurse(1, &cost).1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2673() {
        assert_eq!(Solution::min_increments(7, vec![1, 5, 2, 2, 3, 3, 1]), 6);
        assert_eq!(Solution::min_increments(3, vec![5, 3, 3]), 0);
    }
}
