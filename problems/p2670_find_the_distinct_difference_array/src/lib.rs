pub struct Solution {}

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = vec![false; 51];
        let mut right_count = vec![0; nums.len() + 1];
        let mut count = 0;
        for i in (0..nums.len()).rev() {
            if !seen[nums[i] as usize] {
                seen[nums[i] as usize] = true;
                count += 1;
            }
            right_count[i] = count;
        }

        let mut seen = vec![false; 51];
        let mut result = vec![0; nums.len()];
        let mut count = 0;
        for i in 0..nums.len() {
            if !seen[nums[i] as usize] {
                seen[nums[i] as usize] = true;
                count += 1;
            }
            result[i] = count - right_count[i + 1];
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2670() {
        assert_eq!(
            Solution::distinct_difference_array(vec![1, 2, 3, 4, 5]),
            vec![-3, -1, 1, 3, 5]
        );
        assert_eq!(
            Solution::distinct_difference_array(vec![3, 2, 3, 4, 2]),
            vec![-2, -1, 0, 2, 3]
        );
    }
}
