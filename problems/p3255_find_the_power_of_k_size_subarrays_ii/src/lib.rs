pub struct Solution {}

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut count = 0;
        for i in 0..nums.len() {
            if 0 < i && nums[i - 1] + 1 != nums[i] {
                count = 1;
            } else {
                count += 1;
            }
            if k - 1 <= i as i32 {
                result.push(if k <= count { nums[i] } else { -1 });
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3355() {
        assert_eq!(
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        );
        assert_eq!(
            Solution::results_array(vec![2, 2, 2, 2, 2], 4),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2),
            vec![-1, 3, -1, 3, -1]
        );
    }
}
