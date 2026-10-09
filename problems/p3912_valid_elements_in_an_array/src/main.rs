pub struct Solution {}

impl Solution {
    pub fn find_valid_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut right = vec![0; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            right[i] = right[i + 1].max(nums[i + 1]);
        }
        let mut left = vec![0; nums.len() + 1];
        let mut result = Vec::new();

        for i in 0..nums.len() {
            if left[i] < nums[i] || right[i] < nums[i] {
                result.push(nums[i]);
            }
            left[i + 1] = left[i].max(nums[i])
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3912() {
        assert_eq!(
            Solution::find_valid_elements(vec![1, 2, 4, 2, 3, 2]),
            vec![1, 2, 4, 3, 2]
        );
        assert_eq!(Solution::find_valid_elements(vec![5, 5, 5, 5]), vec![5, 5]);
        assert_eq!(Solution::find_valid_elements(vec![1]), vec![1]);
    }
}

fn main() {}
