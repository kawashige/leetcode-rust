pub struct Solution {}

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut count = vec![0; 32];
        for i in 0..nums.len() {
            for j in 0..32 {
                if nums[i] & 1 << j != 0 {
                    count[j] += 1;
                    if k <= count[j] {
                        result |= 1 << j;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2917() {
        assert_eq!(Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4), 9);
        assert_eq!(Solution::find_k_or(vec![2, 12, 1, 11, 4, 5], 6), 0);
        assert_eq!(Solution::find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1), 15);
    }
}
