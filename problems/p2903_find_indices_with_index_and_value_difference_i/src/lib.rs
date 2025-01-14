pub struct Solution {}

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        if nums.len() > index_difference as usize {
            for i in 0..nums.len() - index_difference as usize {
                for j in i + index_difference as usize..nums.len() {
                    if value_difference <= (nums[i] - nums[j]).abs() {
                        return vec![i as i32, j as i32];
                    }
                }
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2903() {
        assert_eq!(Solution::find_indices(vec![0], 100, 50), vec![-1, -1]);
        assert_eq!(Solution::find_indices(vec![5, 1, 4, 1], 2, 4), vec![0, 3]);
        assert_eq!(Solution::find_indices(vec![2, 1], 0, 0), vec![0, 0]);
        assert_eq!(Solution::find_indices(vec![1, 2, 3], 2, 4), vec![-1, -1]);
    }
}
