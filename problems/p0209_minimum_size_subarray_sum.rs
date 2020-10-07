pub struct Solution {}

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = nums.len() + 1;
        let mut j = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            if sum < s {
                continue;
            }

            while s <= sum - nums[j] {
                sum -= nums[j];
                j += 1;
            }
            min = std::cmp::min(min, i - j + 1);
        }

        if sum < s {
            0
        } else {
            min as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0209() {
        assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
        assert_eq!(1, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 7]));
        assert_eq!(0, Solution::min_sub_array_len(100, vec![2, 3, 1, 2, 4, 7]));
        assert_eq!(3, Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]));
    }
}
