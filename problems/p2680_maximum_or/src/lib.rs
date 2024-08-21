pub struct Solution {}

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = vec![0; 32];
        let mut num = 0;
        for i in 0..nums.len() {
            for j in 0..32 {
                if nums[i] & 1 << j != 0 {
                    count[j] += 1;
                }
            }
            num |= nums[i] as i64;
        }

        let mut result = 0;
        for i in 0..nums.len() {
            let mut current = num;
            for j in 0..32 {
                if nums[i] & 1 << j != 0 && count[j] == 1 {
                    current ^= 1 << j;
                }
            }
            result = result.max(current | ((nums[i] as i64) << k))
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2680() {
        assert_eq!(Solution::maximum_or(vec![94, 35], 2), 379);
        assert_eq!(Solution::maximum_or(vec![12, 9], 1), 30);
        assert_eq!(Solution::maximum_or(vec![8, 1, 2], 2), 35);
    }
}
