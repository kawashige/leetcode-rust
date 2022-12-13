use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        const M: i32 = 1_000_000_007;
        let mut counts = HashMap::new();
        let mut count = 0;

        for num in nums {
            let rev = num
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            *counts.entry(num - rev).or_insert(0) += 1;
            count = (count + counts[&(num - rev)] - 1) % M;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1814() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
        assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
    }
}
