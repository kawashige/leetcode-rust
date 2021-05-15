pub struct Solution {}

impl Solution {
    pub fn is_valid_integer(s: &str) -> bool {
        let s = if s.starts_with("+") || s.starts_with("-") {
            &s[1..]
        } else {
            s
        };

        !s.is_empty() && s.chars().all(char::is_numeric)
    }

    pub fn is_valid_decimal(s: &str) -> bool {
        let split_dot = if s.starts_with("+") || s.starts_with("-") {
            &s[1..]
        } else {
            s
        }
        .split('.')
        .collect::<Vec<&str>>();

        split_dot.len() == 2
            && !(split_dot[0].is_empty() && split_dot[1].is_empty())
            && split_dot[0].chars().all(char::is_numeric)
            && split_dot[1].chars().all(char::is_numeric)
    }

    pub fn is_number(s: String) -> bool {
        let split_e = s.split(|c| c == 'e' || c == 'E').collect::<Vec<&str>>();
        if split_e.len() > 2 || split_e.iter().any(|s| s.is_empty()) {
            return false;
        }

        (split_e.len() == 1 || Self::is_valid_integer(split_e[1]))
            && if split_e[0].contains('.') {
                Self::is_valid_decimal(split_e[0])
            } else {
                Self::is_valid_integer(split_e[0])
            }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day15() {
        assert!(Solution::is_number("0".to_string()));
        assert!(!Solution::is_number("e".to_string()));
        assert!(!Solution::is_number(".".to_string()));
        assert!(Solution::is_number("0.1".to_string()));

        for s in [
            "2",
            "0089",
            "-0.1",
            "+3.14",
            "4.",
            "-.9",
            "2e10",
            "-90E3",
            "3e+7",
            "+6e-1",
            "53.5e93",
            "-123.456e789",
        ]
        .iter()
        {
            assert!(Solution::is_number(s.to_string()));
        }

        for s in ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"].iter() {
            println!("s: {}", s);
            assert!(!Solution::is_number(s.to_string()));
        }
    }
}
