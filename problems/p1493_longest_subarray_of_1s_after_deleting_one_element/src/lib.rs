pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut compress = Vec::new();
        let mut count = 0;

        for i in 0..nums.len() {
            count += 1;
            if i == nums.len() - 1 || nums[i] != nums[i + 1] {
                compress.push((nums[i], count));
                count = 0;
            }
        }

        let mut longest = 0;

        for i in 0..compress.len() {
            if compress[i].0 == 1 {
                longest = longest.max(compress[i].1);
            } else if (1..compress.len() - 1).contains(&i) && compress[i].1 == 1 {
                longest = longest.max(compress[i - 1].1 + compress[i + 1].1)
            }
        }

        if longest == nums.len() {
            longest -= 1;
        }

        longest as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1493() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        );
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
    }
}
