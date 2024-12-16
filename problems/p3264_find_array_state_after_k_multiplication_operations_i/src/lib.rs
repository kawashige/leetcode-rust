pub struct Solution {}

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut nums = nums;
        for _ in 0..k {
            let mut min = nums[0];
            let mut min_i = 0;
            for i in 1..nums.len() {
                if nums[i] < min {
                    min = nums[i];
                    min_i = i;
                }
            }
            nums[min_i] *= multiplier
        }
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3264() {
        assert_eq!(
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
            vec![8, 4, 6, 5, 6]
        );
        assert_eq!(Solution::get_final_state(vec![1, 2], 3, 4), vec![16, 8]);
    }
}
