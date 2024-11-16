pub struct Solution {}

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);
        for i in k - 1..nums.len() {
            if k - 1 < i && result.last() != Some(&-1) && nums[i - 1] + 1 == nums[i] {
                result.push(nums[i]);
                continue;
            }
            let mut is_consecutive = true;
            for j in 0..k - 1 {
                if nums[i - j - 1] + 1 != nums[i - j] {
                    is_consecutive = false;
                    break;
                }
            }
            result.push(if is_consecutive { nums[i] } else { -1 });
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3254() {
        assert_eq!(Solution::results_array(vec![2, 3], 2), vec![3]);
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
