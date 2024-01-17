pub struct Solution {}

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        ((amount.iter().sum::<i32>() + 1) / 2).max(*amount.iter().max().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2335() {
        assert_eq!(Solution::fill_cups(vec![1, 4, 2]), 4);
        assert_eq!(Solution::fill_cups(vec![5, 4, 4]), 7);
        assert_eq!(Solution::fill_cups(vec![5, 0, 0]), 5);
    }
}
