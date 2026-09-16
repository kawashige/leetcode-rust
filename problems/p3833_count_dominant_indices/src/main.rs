pub struct Solution {}

impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut sum = *nums.last().unwrap();
        let mut result = 0;
        for i in (0..nums.len() - 1).rev() {
            if sum < nums[i] * (nums.len() - 1 - i) as i32 {
                result += 1;
            }
            sum += nums[i];
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3833() {
        assert_eq!(Solution::dominant_indices(vec![5, 4, 3]), 2);
        assert_eq!(Solution::dominant_indices(vec![4, 1, 2]), 1);
    }
}

fn main() {}
