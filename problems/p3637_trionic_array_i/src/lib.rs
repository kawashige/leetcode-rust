pub struct Solution {}

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut cnt = 0;
        for i in 0..nums.len() {
            if 0 < i && nums[i - 1] == nums[i] {
                return false;
            }
            if 1 < i && nums[i].cmp(&nums[i - 1]) != nums[i - 1].cmp(&nums[i - 2]) {
                cnt += 1;
            }
        }

        nums[0] < nums[1] && cnt == 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3637() {
        assert!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]));
        assert!(!Solution::is_trionic(vec![2, 1, 3]));
    }
}
