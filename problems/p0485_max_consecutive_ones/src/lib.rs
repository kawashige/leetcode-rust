pub struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let l = nums.len();
        let mut i = 0;
        while i < nums.len() {
            if let Some(j) = (i..l).find(|i| nums[*i] == 1) {
                let k = ((j + 1)..l).find(|i| nums[*i] == 0).unwrap_or(l);
                result = std::cmp::max(result, k - j);
                i = k + 1;
            } else {
                break;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0485() {
        assert_eq!(
            3,
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        );
        assert_eq!(
            0,
            Solution::find_max_consecutive_ones(vec![0, 0, 0, 0, 0, 0]),
        );
        assert_eq!(0, Solution::find_max_consecutive_ones(vec![]),)
    }
}
