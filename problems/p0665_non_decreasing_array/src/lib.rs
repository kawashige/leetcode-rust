pub struct Solution {}

impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut modified = false;
        for i in 0..(nums.len() - 1) {
            if nums[i] <= nums[i + 1] {
                continue;
            }

            if modified {
                return false;
            }

            modified = true;

            if i != 0 && i != nums.len() - 2 {
                if nums[i] <= nums[i + 2] {
                    nums[i + 1] = nums[i + 2]
                } else if nums[i - 1] > nums[i + 1] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0665() {
        assert!(Solution::check_possibility(vec![1, 3, 2]));
        assert!(Solution::check_possibility(vec![4, 2, 3]));
        assert!(!Solution::check_possibility(vec![4, 2, 1]));
    }
}
