pub struct Solution {}

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;

        for i in 0..arr.len() as i32 {
            let mut count = 0;
            for j in (1..=arr.len() as i32).step_by(2) {
                count += (i + j - 1).min(arr.len() as i32 - 1) - (i - j + 1).max(0) + 1 - (j - 1);
            }
            sum += count * arr[i as usize] as i32;
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1588() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
    }
}
