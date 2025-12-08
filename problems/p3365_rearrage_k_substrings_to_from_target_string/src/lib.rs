use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let mut count = HashMap::new();
        let k = k as usize;
        let l = s.len() / k;

        for i in 0..k {
            *count.entry(s[i * l..(i + 1) * l].to_string()).or_insert(0) += 1;
        }

        for i in 0..k {
            if let Some(x) = count.get_mut(&t[i * l..(i + 1) * l]) {
                *x -= 1;
                if x == &0 {
                    count.remove(&t[i * l..(i + 1) * l]);
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3365() {
        assert!(Solution::is_possible_to_rearrange(
            "abcd".to_string(),
            "cdab".to_string(),
            2
        ));
        assert!(Solution::is_possible_to_rearrange(
            "aabbcc".to_string(),
            "bbaacc".to_string(),
            3
        ));
        assert!(!Solution::is_possible_to_rearrange(
            "aabbcc".to_string(),
            "bbaacc".to_string(),
            2
        ));
    }
}
