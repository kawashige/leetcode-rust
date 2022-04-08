use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut state = 0;
        let mut result = 0;
        map.insert(result, 0);

        for (i, b) in s.as_bytes().iter().enumerate() {
            if [b'a', b'e', b'i', b'o', b'u'].contains(b) {
                state ^= 1 << (*b - b'a') as usize;
            }
            if let Some(j) = map.get(&state) {
                result = result.max(i - j + 1);
            } else {
                map.insert(state, i + 1);
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1371() {
        assert_eq!(
            Solution::find_the_longest_substring("eleetminicoworoep".to_string()),
            13
        );
        assert_eq!(
            Solution::find_the_longest_substring("leetcodeisgreat".to_string()),
            5
        );
        assert_eq!(
            Solution::find_the_longest_substring("bcbcbc".to_string()),
            6
        );
    }
}
