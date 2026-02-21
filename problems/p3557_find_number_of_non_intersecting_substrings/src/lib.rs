use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_substrings(word: String) -> i32 {
        let mut last_indices = vec![-1; 26];
        let mut queue = VecDeque::new();
        let mut dp = vec![0; word.len()];

        for i in 0..word.len() {
            if queue.len() == 3 {
                let (c, j) = queue.pop_front().unwrap();
                last_indices[c] = j as i32;
            }
            if 0 < i {
                dp[i] = dp[i - 1];
            }
            let c = (word.as_bytes()[i] - b'a') as usize;
            if last_indices[c] != -1 {
                dp[i] = dp[i].max(if last_indices[c] == 0 {
                    1
                } else {
                    dp[last_indices[c] as usize - 1] + 1
                });
            }
            queue.push_back((c, i));
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3557() {
        assert_eq!(Solution::max_substrings("abcdeafdef".to_string()), 2);
        assert_eq!(Solution::max_substrings("bcdaaaab".to_string()), 1);
    }
}
