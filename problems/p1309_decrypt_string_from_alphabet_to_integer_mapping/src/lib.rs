pub struct Solution {}

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = Vec::new();
        let mut chars = s.chars().collect::<Vec<_>>();

        while let Some(c) = chars.pop() {
            result.push(
                (b'a'
                    + if c == '#' {
                        chars.pop().unwrap().to_digit(10).unwrap() as u8
                            + chars.pop().unwrap().to_digit(10).unwrap() as u8 * 10
                    } else {
                        c.to_digit(10).unwrap() as u8
                    }
                    - 1) as char,
            );
        }

        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1309() {
        assert_eq!(
            Solution::freq_alphabets("10#11#12".to_string()),
            "jkab".to_string()
        );
        assert_eq!(
            Solution::freq_alphabets("1326#".to_string()),
            "acz".to_string()
        );
    }
}
