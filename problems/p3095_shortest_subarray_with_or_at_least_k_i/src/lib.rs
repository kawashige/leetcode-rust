pub struct Solution {}

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = std::usize::MAX;

        for i in 0..nums.len() {
            let mut bitwise_or = 0;
            for j in (0..=i).rev() {
                bitwise_or |= nums[j];
                if k <= bitwise_or {
                    result = result.min(i - j + 1);
                    break;
                }
            }
        }

        if result == std::usize::MAX {
            -1
        } else {
            result as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3095() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
