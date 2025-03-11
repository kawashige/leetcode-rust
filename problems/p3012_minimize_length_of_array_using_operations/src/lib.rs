pub struct Solution {}

impl Solution {
    pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let mut nums = nums;
        nums.sort_unstable();

        if (1..nums.len()).find(|j| nums[*j] % nums[0] != 0).is_some() {
            return 1;
        }

        let mut count = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[0] {
                break;
            }
            count += 1;
        }
        count / 2 + count % 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3012() {
        assert_eq!(Solution::minimum_array_length(vec![1, 4, 3, 1]), 1);
        assert_eq!(Solution::minimum_array_length(vec![5, 5, 5, 10, 5]), 2);
        assert_eq!(Solution::minimum_array_length(vec![2, 3, 4]), 1);
    }
}
