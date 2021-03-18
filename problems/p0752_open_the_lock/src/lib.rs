use std::collections::{HashSet, VecDeque};
pub struct Solution {}

impl Solution {
    pub fn to_digit_array(s: String) -> [u8; 4] {
        let didits = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        [didits[0], didits[1], didits[2], didits[3]]
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends = deadends
            .into_iter()
            .map(|d| Self::to_digit_array(d))
            .collect::<HashSet<[u8; 4]>>();

        if deadends.contains(&[0, 0, 0, 0]) {
            return -1;
        }

        let target = Self::to_digit_array(target);

        let mut queue = VecDeque::new();
        queue.push_back(([0, 0, 0, 0], 0));
        let mut seen = HashSet::new();

        while let Some((n, c)) = queue.pop_front() {
            if n == target {
                return c;
            }

            if deadends.contains(&n) || seen.contains(&n) {
                continue;
            }

            seen.insert(n);

            for i in 0..4 {
                let mut n1 = n.clone();
                let mut n2 = n.clone();
                n1[i] = (n1[i] + 11) % 10;
                n2[i] = (n2[i] + 9) % 10;
                queue.push_back((n1, c + 1));
                queue.push_back((n2, c + 1));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0752() {
        assert_eq!(
            Solution::open_lock(
                vec![
                    "0201".to_string(),
                    "0101".to_string(),
                    "0102".to_string(),
                    "1212".to_string(),
                    "2002".to_string()
                ],
                "0202".to_string()
            ),
            6
        );
        assert_eq!(
            Solution::open_lock(vec!["8888".to_string()], "0009".to_string()),
            1
        );
        assert_eq!(
            Solution::open_lock(
                vec![
                    "8887".to_string(),
                    "8889".to_string(),
                    "8878".to_string(),
                    "8898".to_string(),
                    "8788".to_string(),
                    "8988".to_string(),
                    "7888".to_string(),
                    "9888".to_string()
                ],
                "8888".to_string()
            ),
            -1
        );
        assert_eq!(
            Solution::open_lock(vec!["0000".to_string()], "8888".to_string()),
            -1
        );
    }
}