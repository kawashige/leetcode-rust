pub struct Solution {}

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        (0..nums.len() as i32)
            .find(|i| nums[*i as usize] == i % 10)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2057() {
        assert_eq!(Solution::smallest_equal(vec![0, 1, 2]), 0);
        assert_eq!(Solution::smallest_equal(vec![4, 3, 2, 1]), 2);
        assert_eq!(
            Solution::smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            -1
        );
    }
}
