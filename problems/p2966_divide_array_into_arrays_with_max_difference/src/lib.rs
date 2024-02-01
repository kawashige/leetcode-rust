pub struct Solution {}

impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if nums.len() % 3 != 0 {
            return Default::default();
        }

        let mut result = Vec::with_capacity(nums.len() / 3);
        let mut nums = nums;
        nums.sort_unstable();

        for i in (0..nums.len()).step_by(3) {
            if k < nums[i + 2] - nums[i] {
                return Default::default();
            }
            result.push(nums[i..i + 3].to_vec());
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2966() {
        assert_eq!(
            Solution::divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
            vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]
        );
        assert_eq!(
            Solution::divide_array(vec![1, 3, 3, 2, 7, 3], 3),
            vec![] as Vec<Vec<i32>>
        );
    }
}
