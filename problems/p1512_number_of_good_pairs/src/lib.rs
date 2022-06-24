pub struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut counts = vec![0; 101];

        for i in (0..nums.len()).rev() {
            count += counts[nums[i] as usize];
            counts[nums[i] as usize] += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1512() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
