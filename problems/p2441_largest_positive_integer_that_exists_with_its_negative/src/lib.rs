pub struct Solution {}

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| a.abs().cmp(&b.abs()).then(a.cmp(&b)));
        if let Some(i) = (1..nums.len())
            .rev()
            .find(|i| 0 < nums[*i] && nums[i - 1] < 0 && nums[i - 1].abs() == nums[*i])
        {
            nums[i]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2441() {
        assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
        assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
        assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
    }
}
