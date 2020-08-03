pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort();
        sorted[sorted.len() / 2]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0169() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}
