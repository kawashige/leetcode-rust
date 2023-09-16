pub struct Solution {}

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.push(-2);
        nums.push(1_000_002);
        nums.sort_unstable();

        (1..nums.len() - 1)
            .filter(|i| nums[i - 1] < nums[*i] - 1 && nums[*i] + 1 < nums[i + 1])
            .map(|i| nums[i])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2150() {
        assert_eq!(Solution::find_lonely(vec![10, 6, 5, 8]), vec![8, 10]);
        assert_eq!(Solution::find_lonely(vec![1, 3, 5, 3]), vec![1, 5]);
    }
}
