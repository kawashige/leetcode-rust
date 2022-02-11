pub struct Solution {}

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = vec![0; nums.len() + 1];
        counts[0] = 1;
        let mut count = 0;
        let mut result = 0;

        for i in 0..nums.len() {
            count += nums[i] % 2;
            if k <= count {
                result += counts[(count - k) as usize];
            }
            counts[count as usize] += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1248() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 1, 1, 1], 1), 5);
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
        assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
}
