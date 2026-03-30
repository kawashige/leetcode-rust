pub struct Solution {}

impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = vec![0; k as usize + 1];
        for i in 0..nums.len() / 2 {
            counts[0] += 1;
            counts[(nums[i] - nums[nums.len() - 1 - i]).abs() as usize] -= 1;
            if (nums[i] - nums[nums.len() - 1 - i]).abs() as usize + 1 < counts.len() {
                counts[(nums[i] - nums[nums.len() - 1 - i]).abs() as usize + 1] += 1;
            }
            if (k - nums[i].min(nums[nums.len() - 1 - i]))
                .max(nums[i].max(nums[nums.len() - 1 - i])) as usize
                + 1
                < counts.len()
            {
                counts[(k - nums[i].min(nums[nums.len() - 1 - i]))
                    .max(nums[i].max(nums[nums.len() - 1 - i])) as usize
                    + 1] += 1;
            }
        }
        let mut min = counts[0];
        for i in 1..counts.len() {
            counts[i] += counts[i - 1];
            min = min.min(counts[i]);
        }
        min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3224() {
        assert_eq!(
            Solution::min_changes(vec![9, 2, 7, 7, 8, 9, 1, 5, 1, 9, 4, 9, 4, 7], 9),
            4
        );
        assert_eq!(
            Solution::min_changes(vec![1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0], 1),
            4
        );
        assert_eq!(Solution::min_changes(vec![1, 0, 1, 2, 4, 3], 4), 2);
        assert_eq!(Solution::min_changes(vec![0, 1, 2, 3, 3, 6, 5, 4], 6), 2);
    }
}
