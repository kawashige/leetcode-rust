pub struct Solution {}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut results = Vec::new();
        let mut chars = str.trim_start().chars();
        let mut sign = 1;
        match chars.next() {
            Some(c) => {
                if c == '-' {
                    sign = -1;
                } else if c == '+' {
                    sign = 1;
                } else if c.is_digit(10) {
                    results.push(c)
                } else {
                    return 0;
                }
            }
            None => {
                return 0;
            }
        }

        for c in chars {
            if !c.is_digit(10) {
                break;
            }
            results.push(c);
        }

        if results.len() == 0 {
            return 0;
        }

        match results.iter().collect::<String>().parse::<i32>() {
            Ok(n) => sign * n,
            Err(_) => match sign {
                -1 => std::i32::MIN,
                _ => std::i32::MAX,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0008() {
        assert_eq!(0, Solution::my_atoi("-".to_string()));
        assert_eq!(42, Solution::my_atoi("42".to_string()));
        assert_eq!(4193, Solution::my_atoi("4193 with words".to_string()));
        assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_string()));
    }
}
