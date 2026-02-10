pub struct Solution {}

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 1..nums.len() {
            let mut count = [0; 100_001];
            let mut parity_count = [0; 2];
            for j in (0..=i).rev() {
                count[nums[j] as usize] += 1;
                if count[nums[j] as usize] == 1 {
                    parity_count[(nums[j] as usize) % 2] += 1;
                }
                if parity_count[0] == parity_count[1] {
                    result = result.max(i - j + 1);
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3719() {
        assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
        assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
    }
}
