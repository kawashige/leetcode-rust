pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut operations = 0;
        let mut count = 1;

        for i in 1..=nums.len() {
            if i == nums.len() || nums[i - 1] != nums[i] {
                if count < 2 {
                    return -1;
                }
                let mut remains = count;
                if remains % 2 == 1 {
                    remains -= 3;
                    operations += 1;
                }
                operations += 2 * (remains / 6);
                remains -= 6 * (remains / 6);
                operations += remains / 2;
                count = 0;
            }
            count += 1;
        }
        operations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2870() {
        assert_eq!(
            Solution::min_operations(vec![
                14, 12, 14, 14, 12, 14, 14, 12, 12, 12, 12, 14, 14, 12, 14, 14, 14, 12, 12
            ]),
            7
        );
        assert_eq!(Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
        assert_eq!(Solution::min_operations(vec![2, 1, 2, 2, 3, 3]), -1);
    }
}
