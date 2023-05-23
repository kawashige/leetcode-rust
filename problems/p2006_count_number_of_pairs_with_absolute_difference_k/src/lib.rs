pub struct Solution {}

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if (nums[i] - nums[j]).abs() == k {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2006() {
        assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
        assert_eq!(Solution::count_k_difference(vec![1, 3], 3), 0);
        assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
    }
}
