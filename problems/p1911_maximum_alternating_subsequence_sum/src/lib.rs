pub struct Solution {}

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut sums = [std::i64::MIN, 0];

        for i in 0..nums.len() {
            let mut new_sums = sums.clone();
            if sums[0] != std::i64::MIN {
                new_sums[1] = new_sums[1].max(sums[0] - nums[i] as i64);
            }
            if sums[1] != std::i64::MIN {
                new_sums[0] = new_sums[0].max(sums[1] + nums[i] as i64);
            }
            sums = new_sums;
        }

        sums[0].max(sums[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1911() {
        assert_eq!(Solution::max_alternating_sum(vec![4, 2, 5, 3]), 7);
        assert_eq!(Solution::max_alternating_sum(vec![5, 6, 7, 8]), 8);
        assert_eq!(Solution::max_alternating_sum(vec![6, 2, 1, 2, 4, 5]), 10);
    }
}
