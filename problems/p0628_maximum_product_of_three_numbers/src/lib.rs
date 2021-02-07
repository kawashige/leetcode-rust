pub struct Solution {}

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let l = nums.len() - 1;
        nums.sort_unstable();
        if nums[0] < 0 && nums[1] < 0 && 0 < nums[l] {
            std::cmp::max(
                nums[0] * nums[1] * nums[l],
                nums[l - 2] * nums[l - 1] * nums[l],
            )
        } else {
            nums[l - 2] * nums[l - 1] * nums[l]
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0628() {
        assert_eq!(Solution::maximum_product(vec![-10, -20, 3, 10]), 2000);
        assert_eq!(Solution::maximum_product(vec![-10, 1, 10, 20]), 200);
        assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
        assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6);
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3, 1]), 6);
    }
}
