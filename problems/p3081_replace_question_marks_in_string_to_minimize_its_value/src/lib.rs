pub struct Solution {}

impl Solution {
    pub fn minimize_string_value(s: String) -> String {
        let mut count = [0; 26];
        let mut targets = 0;
        for i in 0..s.len() {
            if s.as_bytes()[i] != b'?' {
                count[(s.as_bytes()[i] - b'a') as usize] += 1;
            } else {
                targets += 1;
            }
        }
        let mut tmp = Vec::new();
        for _ in 0..targets {
            if let Some(j) = (0..26).min_by_key(|j| count[*j as usize]) {
                tmp.push(j);
                count[j as usize] += 1;
            }
        }

        tmp.sort_unstable();
        let mut j = 0;
        let mut result = String::new();
        for i in 0..s.len() {
            if s.as_bytes()[i] == b'?' {
                result.push((tmp[j] + b'a') as char);
                j += 1;
            } else {
                result.push(s.as_bytes()[i] as char);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3081() {
        assert_eq!(
            Solution::minimize_string_value("???".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            Solution::minimize_string_value("a?a?".to_string()),
            "abac".to_string()
        );
    }
}
