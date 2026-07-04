pub struct Solution {}

impl Solution {
    pub fn count_elements(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return nums.len() as i32;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut count = 1;

        for i in (0..nums.len() - 1).rev() {
            if nums[i] != nums[i + 1] && k <= count {
                return i as i32 + 1;
            }
            count += 1;
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3759() {
        assert_eq!(Solution::count_elements(vec![3, 1, 2], 1), 2);
        assert_eq!(Solution::count_elements(vec![5, 5, 5], 2), 0);
    }
}

fn main() {}
