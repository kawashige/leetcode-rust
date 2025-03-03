use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        let mut is_ok = -1;
        let mut result = vec![];
        let mut candidate = VecDeque::new();

        for i in 0..s.len() {
            if s[i..].starts_with(&a) {
                if (i as i32) <= is_ok {
                    result.push(i as i32);
                } else {
                    candidate.push_back(i as i32);
                }
            }
            if s[i..].starts_with(&b) {
                while let Some(j) = candidate.pop_front() {
                    if i as i32 - j <= k {
                        result.push(j);
                    }
                }
                is_ok = i as i32 + k;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3006() {
        assert_eq!(
            Solution::beautiful_indices(
                "isawsquirrelnearmysquirrelhouseohmy".to_string(),
                "my".to_string(),
                "squirrel".to_string(),
                15
            ),
            vec![16, 33]
        );
        assert_eq!(
            Solution::beautiful_indices("abcd".to_string(), "a".to_string(), "a".to_string(), 4),
            vec![0]
        );
    }
}
