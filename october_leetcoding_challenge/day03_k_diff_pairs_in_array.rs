pub struct Solution {}

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut result = 0;
        for i in 0..nums.len() {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            for j in (i + 1)..nums.len() {
                let diff = (nums[j] - nums[i]).abs();
                if diff == k {
                    result += 1;
                    break;
                } else if diff > k {
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day3() {
        assert_eq!(2, Solution::find_pairs(vec![3, 1, 4, 1, 5], 2));
        assert_eq!(4, Solution::find_pairs(vec![1, 2, 3, 4, 5], 1));
        assert_eq!(1, Solution::find_pairs(vec![3, 1, 4, 1, 5], 0));
        assert_eq!(
            2,
            Solution::find_pairs(vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3)
        );
        assert_eq!(2, Solution::find_pairs(vec![-1, -2, -3], 1));
    }
}
