use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
        let mut diff = 0;
        let mut xor = 0;
        let mut map = HashMap::new();
        map.insert((xor, diff), -1);
        let mut result = 0;

        for i in 0..nums.len() {
            diff += if nums[i] % 2 == 0 { 1 } else { -1 };
            xor ^= nums[i];

            if let Some(j) = map.get(&(xor, diff)) {
                result = result.max(i as i32 - j);
            } else {
                map.insert((xor, diff), i as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3755() {
        assert_eq!(Solution::max_balanced_subarray(vec![3, 1, 3, 2, 0]), 4);
        assert_eq!(
            Solution::max_balanced_subarray(vec![3, 2, 8, 5, 4, 14, 9, 15]),
            8
        );
        assert_eq!(Solution::max_balanced_subarray(vec![0]), 0);
    }
}

fn main() {}
