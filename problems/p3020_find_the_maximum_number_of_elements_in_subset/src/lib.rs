use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let count = nums.iter().fold(HashMap::new(), |mut count, num| {
            *count.entry(*num).or_insert(0) += 1;
            count
        });
        let mut seen = HashSet::new();
        let mut result = 0;
        let mut nums = nums;
        nums.sort_unstable();

        for num in nums {
            if seen.contains(&num) {
                continue;
            }
            if num == 1 {
                result = count[&num] - if count[&num] % 2 == 0 { 1 } else { 0 };
                seen.insert(num);
                continue;
            }
            let mut l = 0;
            let mut x = num as i32;
            while let Some(c) = count.get(&x) {
                seen.insert(x);
                if c == &1 {
                    l += 1;
                    break;
                }
                l += 2;
                x = x * x;
            }
            result = result.max(l - if 0 < l && l % 2 == 0 { 1 } else { 0 });
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3020() {
        assert_eq!(Solution::maximum_length(vec![5, 4, 1, 2, 2]), 3);
        assert_eq!(Solution::maximum_length(vec![1, 3, 2, 4]), 1);
    }
}
