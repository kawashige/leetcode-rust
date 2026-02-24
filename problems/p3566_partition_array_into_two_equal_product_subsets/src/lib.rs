pub struct Solution {}

impl Solution {
    pub fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        for i in 1..2_usize.pow(nums.len() as u32) {
            let mut val = [1_i64; 2];
            for j in 0..nums.len() {
                val[(i & 1 << j == 0) as usize] =
                    val[(i & 1 << j == 0) as usize].saturating_mul(nums[j] as i64);
            }
            if val[0] == target && val[1] == target {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3566() {
        assert!(Solution::check_equal_partitions(vec![3, 1, 6, 8, 4], 24));
        assert!(!Solution::check_equal_partitions(vec![2, 5, 3, 7], 15));
    }
}
