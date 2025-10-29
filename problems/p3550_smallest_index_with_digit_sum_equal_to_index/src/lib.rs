pub struct Solution {}

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        (0..nums.len() as i32)
            .find(|i| {
                nums[*i as usize]
                    .to_string()
                    .chars()
                    .map(|c| (c as u8 - b'0') as i32)
                    .sum::<i32>()
                    == *i
            })
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3550() {
        assert_eq!(Solution::smallest_index(vec![1, 3, 2]), 2);
        assert_eq!(Solution::smallest_index(vec![1, 10, 11]), 1);
        assert_eq!(Solution::smallest_index(vec![1, 2, 3]), -1);
    }
}
