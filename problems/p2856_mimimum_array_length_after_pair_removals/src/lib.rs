pub struct Solution {}

impl Solution {
    pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {
        if nums[0] == nums[nums.len() - 1] {
            return nums.len() as i32;
        }
        let mut c = 1;
        let mut max_c = 1;
        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] {
                max_c = max_c.max(c);
                c = 0;
            }
            c += 1;
        }
        max_c = max_c.max(c);
        println!("{}", max_c);
        if nums.len() - max_c < max_c {
            (max_c - (nums.len() - max_c)) as i32
        } else {
            nums.len() as i32 % 2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2856() {
        assert_eq!(
            Solution::min_length_after_removals(vec![1, 1, 1, 1, 1, 1, 1, 2, 2]),
            5
        );
        assert_eq!(Solution::min_length_after_removals(vec![1, 2, 3, 4]), 0);
        assert_eq!(
            Solution::min_length_after_removals(vec![1, 1, 2, 2, 3, 3]),
            0
        );
        assert_eq!(Solution::min_length_after_removals(vec![2, 3, 4, 4, 4]), 1);
        assert_eq!(
            Solution::min_length_after_removals(vec![1000000000, 1000000000]),
            2
        );
    }
}
