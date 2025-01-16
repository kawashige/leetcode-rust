pub struct Solution {}

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            let mut found = vec![false; 101];
            let mut count = 0;
            for j in (0..=i).rev() {
                if !found[nums[j] as usize] {
                    count += 1;
                    found[nums[j] as usize] = true;
                }
                result += count * count;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2913() {
        assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
        assert_eq!(Solution::sum_counts(vec![1, 1]), 3);
    }
}
