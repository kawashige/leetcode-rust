pub struct Solution {}

impl Solution {
    pub fn count_ways(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut count = 0;
        if nums[0] != 0 {
            count += 1;
        }

        for i in 0..nums.len() {
            if i < nums.len() - 1 && nums[i] == nums[i + 1] {
                continue;
            }
            if nums[i] < i as i32 + 1 && (i == nums.len() - 1 || i as i32 + 1 < nums[i + 1]) {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2860() {
        assert_eq!(Solution::count_ways(vec![1, 1]), 2);
        assert_eq!(Solution::count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]), 3);
    }
}
