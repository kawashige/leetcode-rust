pub struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] && (i * j) as i32 % k == 0 {
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
    fn test_2176() {
        assert_eq!(Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
        assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 1), 0);
    }
}
