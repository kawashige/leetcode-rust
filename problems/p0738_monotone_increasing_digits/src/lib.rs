pub struct Solution {}

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits = n
            .to_string()
            .chars()
            .rev()
            .map(|c| c as usize - 0x30)
            .collect::<Vec<usize>>();

        for i in 0..digits.len() {
            if digits[i] == 9 {
                continue;
            }
            if let Some(j) = ((i + 1)..digits.len()).find(|j| digits[i] < digits[*j]) {
                for k in 0..j {
                    digits[k] = 9;
                }
                digits[j] -= 1;
            }
        }

        while digits.last() == Some(&0) {
            digits.pop();
        }

        digits
            .into_iter()
            .rev()
            .map(|d| (d as u8 + 0x30) as char)
            .collect::<String>()
            .parse()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_00738() {
        assert_eq!(Solution::monotone_increasing_digits(100), 99);
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
        assert_eq!(Solution::monotone_increasing_digits(332), 299);
    }
}
