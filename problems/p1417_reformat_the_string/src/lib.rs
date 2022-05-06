pub struct Solution {}

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut digit = Vec::new();
        let mut alpha = Vec::new();

        for c in s.chars() {
            if c.is_ascii_alphabetic() {
                alpha.push(c);
            } else {
                digit.push(c)
            }
        }

        let mut s = String::new();

        if 1 < (alpha.len() as i32 - digit.len() as i32).abs() {
            return s;
        }

        if alpha.len() < digit.len() {
            while !alpha.is_empty() || !digit.is_empty() {
                s.push(digit.pop().unwrap());
                if !alpha.is_empty() {
                    s.push(alpha.pop().unwrap());
                }
            }
        } else {
            while !alpha.is_empty() || !digit.is_empty() {
                s.push(alpha.pop().unwrap());
                if !digit.is_empty() {
                    s.push(digit.pop().unwrap());
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1417() {
        assert_eq!(
            Solution::reformat("covid2019".to_string()),
            "d9i1v0o2c".to_string()
        );
        assert_eq!(
            Solution::reformat("a0b1c2".to_string()),
            "c2b1a0".to_string()
        );
        assert_eq!(Solution::reformat("leetcode".to_string()), "".to_string());
        assert_eq!(Solution::reformat("1229857369".to_string()), "".to_string());
    }
}
