use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        if s.as_bytes().last() == Some(&b'1') {
            return false;
        }

        let mut deque = VecDeque::new();
        deque.push_back(0);

        for i in 1..s.len() {
            if s.as_bytes()[i] == b'1' {
                continue;
            }
            while !deque.is_empty() && max_jump as usize + deque[0] < i {
                deque.pop_front();
            }

            if !deque.is_empty() && min_jump as usize + deque[0] <= i {
                if i == s.len() - 1 {
                    return true;
                } else {
                    deque.push_back(i);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1871() {
        assert!(Solution::can_reach("011010".to_string(), 2, 3));
        assert!(!Solution::can_reach("01101110".to_string(), 2, 3));
    }
}
