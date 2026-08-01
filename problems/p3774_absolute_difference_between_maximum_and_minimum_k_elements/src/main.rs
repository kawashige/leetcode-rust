pub struct Solution {}

impl Solution {
    pub fn abs_difference(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut nums = nums;
        nums.sort_unstable();
        (nums[..k].iter().sum::<i32>() - nums[nums.len() - k..].iter().sum::<i32>()).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3774() {
        assert_eq!(Solution::abs_difference(vec![5, 2, 2, 4], 2), 5);
        assert_eq!(Solution::abs_difference(vec![100], 1), 0);
    }
}

fn main() {}
