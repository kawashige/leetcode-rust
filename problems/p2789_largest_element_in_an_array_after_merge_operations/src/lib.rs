pub struct Solution {}

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut i = nums.len() - 1;
        let mut result = nums[0] as i64;

        while 0 < i {
            let mut num = nums[i] as i64;
            let mut j = i - 1;
            while nums[j] as i64 <= num {
                num += nums[j] as i64;
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            result = result.max(num);
            i = j
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2789() {
        assert_eq!(Solution::max_array_value(vec![2, 3, 7, 9, 3]), 21);
        assert_eq!(Solution::max_array_value(vec![5, 3, 3]), 11);
    }
}
