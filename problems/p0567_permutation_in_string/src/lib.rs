use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1_counts = s1.chars().fold([0; 26], |mut counts, c| {
            counts[c as usize - 0x61] += 1;
            counts
        });

        let mut s2_counts = [0; 26];
        let mut queue = VecDeque::new();
        for c in s2.chars() {
            let i = c as usize - 0x61;
            s2_counts[i] += 1;
            queue.push_back(i);
            if queue.len() > s1.len() {
                let removed = queue.pop_front().unwrap();
                s2_counts[removed] -= 1;
            }
            if s1_counts == s2_counts {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0567() {
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string(),
        ));
        assert!(!Solution::check_inclusion(
            "ab".to_string(),
            "eidboaoo".to_string(),
        ));
        assert!(!Solution::check_inclusion("a".to_string(), "a".to_string(),));
    }
}
