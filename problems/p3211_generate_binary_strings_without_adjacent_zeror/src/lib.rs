pub struct Solution {}

impl Solution {
    pub fn recurse(s: &mut String, n: usize, result: &mut Vec<String>) {
        if s.len() == n {
            result.push(s.clone());
            return;
        }

        s.push('1');
        Self::recurse(s, n, result);
        s.pop();
        if s.is_empty() || s.as_bytes()[s.len() - 1] == b'1' {
            s.push('0');
            Self::recurse(s, n, result);
            s.pop();
        }
    }

    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        Self::recurse(&mut String::new(), n as usize, &mut result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3211() {
        assert_eq!(
            Solution::valid_strings(3),
            vec![
                "010".to_string(),
                "011".to_string(),
                "101".to_string(),
                "110".to_string(),
                "111".to_string()
            ]
        );
        assert_eq!(
            Solution::valid_strings(1),
            vec!["0".to_string(), "1".to_string()]
        );
    }
}
