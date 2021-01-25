pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sums = vec![0; nums.len() + 1];
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            sums[i + 1] = sum;
        }

        let mut count = 0;
        for i in 1..sums.len() {
            for j in 0..i {
                if sums[i] - sums[j] == k {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0560() {
        assert_eq!(Solution::subarray_sum(vec![-1, -1, 1], 0), 1);
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
