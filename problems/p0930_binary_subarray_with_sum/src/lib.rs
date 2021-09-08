pub struct Solution {}

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut acc = vec![0; nums.len()];
        let mut count = vec![0; nums.len() + goal as usize + 1];

        for i in 0..nums.len() {
            acc[i] += nums[i];
            if i > 0 {
                acc[i] += acc[i - 1];
            }
            count[acc[i] as usize] += 1;
        }

        let mut r = 0;
        for i in 0..nums.len() {
            r += count[(goal + if i == 0 { 0 } else { acc[i - 1] }) as usize];
            count[acc[i] as usize] -= 1;
        }
        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0930() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 1, 1, 1, 1], 3), 3);
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }
}
