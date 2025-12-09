pub struct Solution {}

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; 100_001];
        let mut left = vec![0; 100_001];
        let mut right = vec![0; 100_001];

        for i in (0..nums.len()).rev() {
            if nums[i] * 2 < count.len() as i32 {
                right[i] = count[nums[i] as usize * 2];
            }
            count[nums[i] as usize] += 1;
        }

        let mut result = 0;
        for i in 0..nums.len() {
            if nums[i] * 2 < left.len() as i32 {
                result += left[nums[i] as usize * 2] as i64 * right[i] as i64;
                result %= 1_000_000_007;
            }
            left[nums[i] as usize] += 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3585() {
        assert_eq!(Solution::special_triplets(vec![6, 3, 6]), 1);
        assert_eq!(Solution::special_triplets(vec![0, 1, 0, 0]), 1);
        assert_eq!(Solution::special_triplets(vec![8, 4, 2, 8, 4]), 2);
    }
}
