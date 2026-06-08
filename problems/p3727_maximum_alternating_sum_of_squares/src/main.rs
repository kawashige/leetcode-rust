pub struct Solution {}

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let l = nums.len();
        let mut nums = nums;
        nums.sort_unstable_by_key(|n| n.abs());

        let mut result = 0;

        for i in 0..l / 2 {
            result +=
                nums[l - 1 - i] as i64 * nums[l - 1 - i] as i64 - nums[i] as i64 * nums[i] as i64;
        }

        if l % 2 == 1 {
            result += nums[l / 2] as i64 * nums[l / 2] as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3727() {
        assert_eq!(Solution::max_alternating_sum(vec![1, 2, 3]), 12);
        assert_eq!(Solution::max_alternating_sum(vec![1, -1, 2, -2, 3, -3]), 16);
    }
}

fn main() {}
