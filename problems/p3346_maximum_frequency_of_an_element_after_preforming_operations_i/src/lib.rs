pub struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut freq = vec![0; 100_001];
        let mut freq2 = vec![0; 100_001];
        for i in 0..nums.len() {
            freq[nums[i] as usize] += 1;
            freq2[if k <= nums[i] {
                (nums[i] - k) as usize
            } else {
                0
            }] += 1;
            if freq2.len() > (k + nums[i] + 1) as usize {
                freq2[(k + nums[i] + 1) as usize] -= 1;
            }
        }

        let mut result = 0;
        for i in 0..freq.len() {
            if 0 < i {
                freq2[i] += freq2[i - 1];
            }
            result = result.max(freq[i] + (freq2[i] - freq[i]).min(num_operations));
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3346() {
        assert_eq!(Solution::max_frequency(vec![1, 4, 5], 1, 2), 2);
        assert_eq!(Solution::max_frequency(vec![5, 11, 20, 20], 5, 1), 2);
    }
}
