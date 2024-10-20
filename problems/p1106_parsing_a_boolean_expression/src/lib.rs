pub struct Solution {}

impl Solution {
    pub fn recurse(e: &str) -> bool {
        if e == "t" {
            true
        } else if e == "f" {
            false
        } else if e.starts_with("!") {
            !Self::recurse(&e[2..e.len() - 1])
        } else if e.starts_with("&") {
            let mut result = true;
            let mut i = 2;
            while i < e.len() - 1 {
                let mut count = 0;
                let mut j = i + 1;
                while j < e.len() {
                    if j == e.len() - 1 || (e.as_bytes()[j] == b',' && count == 0) {
                        result &= Self::recurse(&e[i..j]);
                        i = j + 1;
                        break;
                    } else if e.as_bytes()[j] == b'(' {
                        count += 1;
                    } else if e.as_bytes()[j] == b')' {
                        count -= 1;
                    }
                    j += 1;
                }
            }
            result
        } else if e.starts_with("|") {
            let mut result = false;
            let mut i = 2;
            while i < e.len() - 1 {
                let mut count = 0;
                let mut j = i + 1;
                while j < e.len() {
                    if j == e.len() - 1 || (e.as_bytes()[j] == b',' && count == 0) {
                        result |= Self::recurse(&e[i..j]);
                        i = j + 1;
                        break;
                    } else if e.as_bytes()[j] == b'(' {
                        count += 1;
                    } else if e.as_bytes()[j] == b')' {
                        count -= 1;
                    }
                    j += 1;
                }
            }
            result
        } else {
            unreachable!()
        }
    }

    pub fn parse_bool_expr(expression: String) -> bool {
        Self::recurse(&expression)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1106() {
        assert!(!Solution::parse_bool_expr("&(|(f))".to_string()));
        assert!(Solution::parse_bool_expr("|(f,f,f,t)".to_string()));
        assert!(Solution::parse_bool_expr("!(&(f,t))".to_string()));
    }
}
