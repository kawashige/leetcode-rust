use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        digit: &mut Vec<i32>,
        numbers: &mut HashSet<i32>,
        digits: &[i32],
        seen: &mut Vec<bool>,
    ) {
        if digit.len() == 3 {
            numbers.insert(digit.iter().fold(0, |acc, x| acc * 10 + x));
            return;
        }

        for i in 0..digits.len() {
            if seen[i]
                || (digit.len() == 2 && digits[i] % 2 == 1)
                || (digit.is_empty() && digits[i] == 0)
            {
                continue;
            }
            digit.push(digits[i]);
            seen[i] = true;
            Self::recurse(digit, numbers, digits, seen);
            digit.pop();
            seen[i] = false;
        }
    }
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut numbers = HashSet::new();
        Self::recurse(
            &mut Vec::new(),
            &mut numbers,
            &digits,
            &mut vec![false; digits.len()],
        );
        numbers.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3483() {
        assert_eq!(Solution::total_numbers(vec![1, 2, 3, 4]), 12);
        assert_eq!(Solution::total_numbers(vec![0, 2, 2]), 2);
        assert_eq!(Solution::total_numbers(vec![6, 6, 6]), 1);
        assert_eq!(Solution::total_numbers(vec![1, 3, 5]), 0);
    }
}
