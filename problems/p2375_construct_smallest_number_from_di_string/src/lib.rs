pub struct Solution {}

impl Solution {
    pub fn recurse(digits: &mut Vec<usize>, used: usize, pattern: &str) -> Option<String> {
        if digits.len() == pattern.len() + 1 {
            return Some(digits.iter().map(|d| (*d as u8 + b'0') as char).collect());
        }

        for i in 1..10 {
            if used & 1 << i != 0
                || (!digits.is_empty()
                    && if pattern.as_bytes()[digits.len() - 1] == b'I' {
                        &i < digits.last().unwrap()
                    } else {
                        digits.last().unwrap() < &i
                    })
            {
                continue;
            }
            digits.push(i);
            let result = Self::recurse(digits, used | 1 << i, pattern);
            if result.is_some() {
                return result;
            }
            digits.pop();
        }

        None
    }
    pub fn smallest_number(pattern: String) -> String {
        Self::recurse(&mut Vec::new(), 0, &pattern).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2375() {
        assert_eq!(
            Solution::smallest_number("IIIDIDDD".to_string()),
            "123549876".to_string()
        );
        assert_eq!(
            Solution::smallest_number("DDD".to_string()),
            "4321".to_string()
        );
    }
}
