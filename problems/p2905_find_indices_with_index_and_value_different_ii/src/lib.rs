pub struct Solution {}

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        if index_difference == 0 && value_difference == 0 {
            return vec![0, 0];
        }
        let (mut min_i, mut max_i) = (0, 0);
        let (mut max_v, mut min_v) = (nums[0], nums[0]);
        let index_difference = (index_difference as usize).max(1);

        for i in index_difference..nums.len() {
            if value_difference <= (min_v - nums[i]).abs() {
                return vec![min_i as i32, i as i32];
            }
            if value_difference <= (max_v - nums[i]).abs() {
                return vec![max_i as i32, i as i32];
            }
            if nums[i + 1 - index_difference] < min_v {
                min_i = i + 1 - index_difference;
                min_v = nums[i + 1 - index_difference];
            }
            if nums[i + 1 - index_difference] > max_v {
                max_i = i + 1 - index_difference;
                max_v = nums[i + 1 - index_difference];
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2905() {
        assert_eq!(Solution::find_indices(vec![5, 1, 4, 1], 2, 4), vec![0, 3]);
        assert_eq!(Solution::find_indices(vec![2, 1], 0, 0), vec![0, 0]);
        assert_eq!(Solution::find_indices(vec![1, 2, 3], 2, 4), vec![-1, -1]);
    }
}
