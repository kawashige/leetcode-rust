pub struct Solution {}

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut deletions = 0;

        for i in 0..nums.len() - 1 {
            if (i - deletions) % 2 == 0 && nums[i] == nums[i + 1] {
                deletions += 1;
            }
        }

        deletions as i32
            + if (nums.len() - deletions) % 2 == 1 {
                1
            } else {
                0
            }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2216() {
        assert_eq!(Solution::min_deletion(vec![1, 1, 2, 3, 5]), 1);
        assert_eq!(Solution::min_deletion(vec![1, 1, 2, 2, 3, 3]), 2);
    }
}
