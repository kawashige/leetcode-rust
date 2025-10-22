use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut count = HashMap::new();
        for n in &nums {
            *count.entry(*n).or_insert(0) += 1;
        }
        let mut values = Vec::new();
        for n in &nums {
            values.push(n - k);
            values.push(*n);
            values.push(n + k);
        }
        values.sort_unstable();

        let mut l = 0;
        let mut r = 0;
        let mut result = 0;

        for i in 0..values.len() {
            if 0 < i && values[i] == values[i - 1] {
                continue;
            }
            while nums[l] < values[i] - k {
                l += 1;
            }
            while r + 1 < nums.len() && nums[r + 1] <= values[i] + k {
                r += 1;
            }

            let c = count.get(&values[i]).unwrap_or(&0);
            result = result.max(c + (r as i32 - l as i32 + 1 - c).min(num_operations));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3347() {
        assert_eq!(Solution::max_frequency(vec![1, 4, 5], 1, 2), 2);
        assert_eq!(Solution::max_frequency(vec![5, 11, 20, 20], 5, 1), 2);
    }
}
