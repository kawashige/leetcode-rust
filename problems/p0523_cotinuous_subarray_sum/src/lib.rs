pub struct Solution {}

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        if k == 0 {
            return nums.windows(2).any(|w| w[0] == 0 && w[1] == 0);
        }
        if k == 1 {
            return true;
        }

        let mut sums = vec![0];
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            sums.push(sum);
        }

        for i in 1..sums.len() {
            for j in (i + 1)..sums.len() {
                if (sums[j] - sums[i - 1]) % k == 0 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0523() {
        assert!(Solution::check_subarray_sum(vec![0, 0], 0));
        assert!(!Solution::check_subarray_sum(vec![23], 1));
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 1));
        assert!(!Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 0));
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
        assert!(!Solution::check_subarray_sum(vec![1, 2, 3, 4, 5], 100));
    }
}
