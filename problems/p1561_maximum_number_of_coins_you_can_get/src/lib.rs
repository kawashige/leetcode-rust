pub struct Solution {}

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        ((piles.len() / 3)..piles.len())
            .step_by(2)
            .map(|i| piles[i])
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1561() {
        assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
        assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
        assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }
}
