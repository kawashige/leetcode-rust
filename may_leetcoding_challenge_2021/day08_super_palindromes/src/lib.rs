pub struct Solution {}

impl Solution {
    pub fn dfs(digits: &mut Vec<char>, l: usize, min: u64, max: u64, count: &mut i32) {
        if (l + 1) / 2 == digits.len() {
            let s: String = if l % 2 == 0 {
                digits.iter().chain(digits.iter().rev()).collect()
            } else {
                digits
                    .iter()
                    .chain(digits[..((l + 1) / 2 - 1)].iter().rev())
                    .collect()
            };
            let n = s.parse::<u64>().unwrap();
            let square = n * n;
            if min <= square && square <= max {
                let square_s = square.to_string();
                if square_s == square_s.chars().rev().collect::<String>() {
                    *count += 1;
                }
            }
            return;
        }

        let s = if digits.is_empty() { 1 } else { 0 };
        for i in s..10 {
            digits.push(('0' as u8 + i) as char);
            Self::dfs(digits, l, min, max, count);
            digits.pop();
        }
    }

    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let mut count = 0;
        let min = left.parse::<u64>().unwrap();
        let max = right.parse::<u64>().unwrap();
        for l in 1..=((right.len() + 1) / 2) {
            Self::dfs(
                &mut Vec::with_capacity((l + 1) / 2),
                l,
                min,
                max,
                &mut count,
            );
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day08() {
        assert_eq!(
            Solution::superpalindromes_in_range("4".to_string(), "1000".to_string()),
            4
        );
        assert_eq!(
            Solution::superpalindromes_in_range("1".to_string(), "2".to_string()),
            1
        );
    }
}
