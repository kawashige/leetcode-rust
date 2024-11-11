use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut chars = VecDeque::new();
        let mut is_rev = false;

        for b in s.as_bytes().iter() {
            if b == &b'i' {
                is_rev = !is_rev;
                continue;
            }
            if is_rev {
                chars.push_front(*b as char);
            } else {
                chars.push_back(*b as char);
            }
        }

        if is_rev {
            chars.into_iter().rev().collect()
        } else {
            chars.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2810() {
        assert_eq!(
            Solution::final_string("string".to_string()),
            "rtsng".to_string()
        );
        assert_eq!(
            Solution::final_string("poiinter".to_string()),
            "ponter".to_string()
        );
    }
}
