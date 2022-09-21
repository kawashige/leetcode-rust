pub struct Solution {}

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let digits = number
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>();

        let mut s = String::new();
        let mut i = 0;
        while i < digits.len() {
            if !s.is_empty() {
                s.push('-');
            }
            if i + 4 == digits.len() {
                s += &digits[i..i + 2];
                s.push('-');
                s += &digits[i + 2..i + 4];
                i += 4;
            } else {
                s += &digits[i..(i + 3).min(digits.len())];
                i += 3;
            }
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1694() {
        assert_eq!(
            Solution::reformat_number("12".to_string()),
            "12".to_string()
        );
        assert_eq!(
            Solution::reformat_number("1-23-45 6".to_string()),
            "123-456".to_string()
        );
        assert_eq!(
            Solution::reformat_number("123 4-567".to_string()),
            "123-45-67".to_string()
        );
        assert_eq!(
            Solution::reformat_number("123 4-5678".to_string()),
            "123-456-78".to_string()
        );
    }
}
