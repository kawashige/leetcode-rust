pub struct Solution {}

impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        nums.push(0);
        nums.sort_unstable();
        nums.push(nums[nums.len() - 1] + k + 1);

        let mut sum = 0;
        let mut remains = k;

        for i in 0..nums.len() - 1 {
            if 0 < remains && nums[i] + 1 < nums[i + 1] {
                let inserted = (nums[i + 1] - (nums[i] + 1)).min(remains);
                remains -= inserted;
                sum += (nums[i] as i64 + inserted as i64) * (nums[i] as i64 + inserted as i64 + 1)
                    / 2
                    - nums[i] as i64 * (nums[i] as i64 + 1) / 2;
            }
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2195() {
        assert_eq!(Solution::minimal_k_sum(vec![1, 4, 25, 10, 25], 2), 5);
        assert_eq!(Solution::minimal_k_sum(vec![5, 6], 6), 25);
    }
}
