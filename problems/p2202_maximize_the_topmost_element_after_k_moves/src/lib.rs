pub struct Solution {}

impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            nums[0]
        } else if k % 2 == 1 && nums.len() == 1 {
            -1
        } else if k == 1 {
            nums[1]
        } else if nums.len() == k as usize - 1 || nums.len() == k as usize {
            *nums[0..k as usize - 1].iter().max().unwrap()
        } else if (k as usize) <= nums.len() {
            (*nums[0..k as usize - 1].iter().max().unwrap()).max(nums[k as usize])
        } else {
            *nums.iter().max().unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2202() {
        assert_eq!(Solution::maximum_top(vec![5, 2, 2, 4, 0, 6], 4), 5);
        assert_eq!(Solution::maximum_top(vec![2], 1), -1);
    }
}
