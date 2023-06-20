pub struct Solution {}

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = vec![-1; nums.len()];
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i] as usize;
            sum -= if 2 * k < i {
                nums[i - 2 * k - 1] as usize
            } else {
                0
            };
            if k * 2 <= i {
                result[i - k] = (sum / (2 * k + 1)) as i32;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2090() {
        assert_eq!(
            Solution::get_averages(vec![100000; 100000], 40000),
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1]
        );
        assert_eq!(
            Solution::get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3),
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1]
        );
        assert_eq!(Solution::get_averages(vec![100000], 0), vec![100000]);
        assert_eq!(Solution::get_averages(vec![8], 100000), vec![-1]);
    }
}
