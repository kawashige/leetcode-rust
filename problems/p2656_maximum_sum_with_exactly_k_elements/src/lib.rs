pub struct Solution {}

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().max().unwrap() * k + ((k - 1) * k) / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2656() {
        assert_eq!(Solution::maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
        assert_eq!(Solution::maximize_sum(vec![5, 5, 5], 2), 11);
    }
}
