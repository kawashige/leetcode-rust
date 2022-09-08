pub struct Solution {}

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut sum = vec![vec![0; nums.len() + 1]; 2];
        sum[(nums.len() - 1) % 2][nums.len() - 1] = nums[nums.len() - 1];

        for i in (0..nums.len() - 1).rev() {
            sum[0][i] += sum[0][i + 1];
            sum[1][i] += sum[1][i + 1];
            sum[i % 2][i] += nums[i];
        }

        let mut current_sum = [0; 2];
        let mut count = 0;
        for i in 0..nums.len() {
            if current_sum[0] + sum[1][i + 1] == current_sum[1] + sum[0][i + 1] {
                count += 1;
            }
            current_sum[i % 2] += nums[i];
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1664() {
        assert_eq!(Solution::ways_to_make_fair(vec![2, 1, 6, 4]), 1);
        assert_eq!(Solution::ways_to_make_fair(vec![1, 1, 1]), 3);
        assert_eq!(Solution::ways_to_make_fair(vec![1, 2, 3]), 0);
    }
}
