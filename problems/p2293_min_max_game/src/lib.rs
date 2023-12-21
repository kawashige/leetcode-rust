pub struct Solution {}

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        while 1 < nums.len() {
            nums = (0..nums.len())
                .step_by(2)
                .map(|i| {
                    if (i / 2) % 2 == 0 {
                        nums[i].min(nums[i + 1])
                    } else {
                        nums[i].max(nums[i + 1])
                    }
                })
                .collect()
        }
        nums[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2293() {
        assert_eq!(Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
        assert_eq!(Solution::min_max_game(vec![3]), 3);
    }
}
