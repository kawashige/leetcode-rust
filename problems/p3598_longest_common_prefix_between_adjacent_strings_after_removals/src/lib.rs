use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn prefix_len(s1: &str, s2: &str) -> usize {
        let l = s1.len().min(s2.len());
        (0..l)
            .find(|i| s1.as_bytes()[*i] != s2.as_bytes()[*i])
            .unwrap_or(l)
    }

    pub fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        for i in 1..words.len() {
            let l = Self::prefix_len(&words[i - 1], &words[i]);
            queue.push_front(l);
            for i in 0..queue.len() - 1 {
                if queue[i + 1] < queue[i] {
                    queue.swap(i, i + 1);
                }
            }
            if 3 < queue.len() {
                queue.pop_front();
            }
        }

        let mut result = Vec::new();
        for i in 0..words.len() {
            let mut vals = queue.clone();
            let mut remove = Vec::new();
            if 0 < i {
                remove.push(Self::prefix_len(&words[i - 1], &words[i]));
            }
            if i < words.len() - 1 {
                remove.push(Self::prefix_len(&words[i + 1], &words[i]));
            }
            if 1 < remove.len() && remove[0] < remove[1] {
                remove.swap(0, 1);
            }
            for i in 0..remove.len() {
                if vals.iter().last() == Some(&remove[i]) {
                    vals.pop_back();
                }
            }

            result.push((*vals.iter().last().unwrap_or(&0)).max(
                if (1..words.len() - 1).contains(&i) {
                    Self::prefix_len(&words[i - 1], &words[i + 1])
                } else {
                    0
                },
            ) as i32);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3598() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "jump".to_string(),
                "run".to_string(),
                "run".to_string(),
                "jump".to_string(),
                "run".to_string()
            ]),
            vec![3, 0, 0, 3, 3]
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racer".to_string(),
                "car".to_string()
            ]),
            vec![0, 0, 0]
        );
    }
}
