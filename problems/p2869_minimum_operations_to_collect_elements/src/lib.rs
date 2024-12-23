pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut seen = vec![false; k as usize + 1];
        for i in (0..nums.len()).rev() {
            if nums[i] <= k && !seen[nums[i] as usize] {
                seen[nums[i] as usize] = true;
                count += 1;
                if count == k {
                    return (nums.len() - i) as i32;
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2869() {
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 2), 4);
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 5), 5);
        assert_eq!(Solution::min_operations(vec![3, 2, 5, 3, 1], 3), 4);
    }
}
