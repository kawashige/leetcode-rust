use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        bytes: &[u8],
        current: &mut String,
        substrings: &mut HashSet<String>,
        max_number: &mut usize,
    ) {
        let s = current.to_string();
        if i == bytes.len() {
            if !substrings.contains(&s) {
                *max_number = std::cmp::max(*max_number, substrings.len() + 1);
            }
            return;
        }

        current.push(bytes[i] as char);
        Self::recurse(i + 1, bytes, current, substrings, max_number);
        current.pop();

        if !substrings.contains(&s) && !s.is_empty() {
            substrings.insert(s.to_string());
            current.clear();
            current.push(bytes[i] as char);

            Self::recurse(i + 1, bytes, current, substrings, max_number);

            substrings.remove(&s);
            *current = s;
        }
    }

    pub fn max_unique_split(s: String) -> i32 {
        let mut max_number = 0;

        Self::recurse(
            0,
            s.as_bytes(),
            &mut String::new(),
            &mut HashSet::new(),
            &mut max_number,
        );

        max_number as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1593() {
        assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5);
        assert_eq!(Solution::max_unique_split("aba".to_string()), 2);
        assert_eq!(Solution::max_unique_split("aa".to_string()), 1);
    }
}
