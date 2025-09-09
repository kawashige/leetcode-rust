pub struct Solution {}

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .map(|i| {
                nums[(i as i32
                    + nums.len() as i32
                    + if nums[i] < 0 { -1 } else { 1 } * (nums[i].abs() % nums.len() as i32))
                    as usize
                    % nums.len()]
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3379() {
        assert_eq!(
            Solution::construct_transformed_array(vec![3, -2, 1, 1]),
            vec![1, 1, 1, 3]
        );
        assert_eq!(
            Solution::construct_transformed_array(vec![-1, 4, -1]),
            vec![-1, -1, 4]
        );
    }
}
