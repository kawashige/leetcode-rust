pub struct Solution {}

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = vec![0; nums.len() + 2];
        let mut right_sum = vec![0; nums.len() + 2];
        for i in 0..nums.len() {
            left_sum[i + 1] = left_sum[i] + nums[i];
        }
        for i in (0..nums.len()).rev() {
            right_sum[i + 1] = right_sum[i + 2] + nums[i];
        }

        println!("{:?}", left_sum);
        println!("{:?}", right_sum);

        (0..nums.len() as i32)
            .find(|i| left_sum[*i as usize] == right_sum[*i as usize + 2])
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1991() {
        assert_eq!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]), 3);
        assert_eq!(Solution::find_middle_index(vec![1, -1, 4]), 2);
        assert_eq!(Solution::find_middle_index(vec![2, 5]), -1);
    }
}
