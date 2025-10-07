pub struct Solution {}

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            let mut count = vec![0; 51];
            for i in 0..nums.len() {
                count[nums[i] as usize] += 1;
            }
            (0..count.len() as i32)
                .rev()
                .find(|i| count[*i as usize] == 1)
                .unwrap_or(-1)
        } else if k == nums.len() as i32 {
            nums.into_iter().max().unwrap()
        } else {
            let mut count = vec![0; 51];
            for i in 0..nums.len() {
                count[nums[i] as usize] += 1;
            }
            match (
                count[nums[0] as usize],
                count[nums[nums.len() - 1] as usize],
            ) {
                (1, 1) => nums[0].max(nums[nums.len() - 1]),
                (1, _) => nums[0],
                (_, 1) => nums[nums.len() - 1],
                _ => -1,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3471() {
        assert_eq!(Solution::largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
        assert_eq!(Solution::largest_integer(vec![3, 9, 7, 2, 1, 7], 4), 3);
        assert_eq!(Solution::largest_integer(vec![0, 0], 1), -1);
    }
}
