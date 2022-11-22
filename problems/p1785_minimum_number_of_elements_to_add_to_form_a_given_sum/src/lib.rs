pub struct Solution {}

impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let diff = (goal as i64 - nums.iter().map(|i| *i as i64).sum::<i64>()).abs();
        ((diff + limit as i64 - 1) / limit as i64) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1785() {
        assert_eq!(Solution::min_elements(vec![1, -1, 1], 3, -4), 2);
        assert_eq!(Solution::min_elements(vec![1, -10, 9, 1], 100, 0), 1);
    }
}
