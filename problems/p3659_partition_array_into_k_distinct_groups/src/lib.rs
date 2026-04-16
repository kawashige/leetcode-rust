pub struct Solution {}

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() as i32 % k != 0 {
            return false;
        }
        let group = nums.len() as i32 / k;

        let mut count = vec![0; 100_001];

        for i in 0..nums.len() {
            if count[nums[i] as usize] == group {
                return false;
            }
            count[nums[i] as usize] += 1;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3659() {
        assert!(Solution::partition_array(vec![1, 2, 3, 4], 2));
        assert!(Solution::partition_array(vec![3, 5, 2, 2], 2));
        assert!(!Solution::partition_array(vec![1, 5, 2, 3], 3));
    }
}
