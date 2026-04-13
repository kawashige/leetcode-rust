pub struct Solution {}

impl Solution {
    pub fn perfect_pairs(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable_by_key(|n| n.abs());

        let mut result = 0;
        let mut j = 1;
        for i in 0..nums.len() {
            while j < nums.len() && nums[j].abs() <= 2 * nums[i].abs() {
                j += 1;
            }
            result += (j - i - 1) as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3649() {
        assert_eq!(Solution::perfect_pairs(vec![0, 1, 2, 3]), 2);
        assert_eq!(Solution::perfect_pairs(vec![-3, 2, -1, 4]), 4);
        assert_eq!(Solution::perfect_pairs(vec![1, 10, 100, 1000]), 0);
    }
}
