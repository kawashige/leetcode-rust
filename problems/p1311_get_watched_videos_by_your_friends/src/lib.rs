use std::collections::{HashMap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        level: i32,
    ) -> Vec<String> {
        let mut count = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((id as usize, 0));
        let mut seen = vec![false; friends.len()];
        seen[id as usize] = true;

        while let Some((i, l)) = queue.pop_front() {
            if level == l {
                for video in &watched_videos[i] {
                    *count.entry(video).or_insert(0) += 1;
                }
                continue;
            }

            for next in &friends[i] {
                if seen[*next as usize] {
                    continue;
                }
                seen[*next as usize] = true;
                queue.push_back((*next as usize, l + 1));
            }
        }

        let mut video_counts = count.into_iter().collect::<Vec<_>>();
        video_counts.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        video_counts
            .into_iter()
            .map(|(v, _)| v.to_string())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1311() {
        assert_eq!(
            Solution::watched_videos_by_friends(
                vec![
                    vec!["A".to_string(), "B".to_string()],
                    vec!["C".to_string()],
                    vec!["B".to_string(), "C".to_string()],
                    vec!["D".to_string()]
                ],
                vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
                0,
                1
            ),
            vec!["B".to_string(), "C".to_string()]
        );
        assert_eq!(
            Solution::watched_videos_by_friends(
                vec![
                    vec!["A".to_string(), "B".to_string()],
                    vec!["C".to_string()],
                    vec!["B".to_string(), "C".to_string()],
                    vec!["D".to_string()]
                ],
                vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
                0,
                2
            ),
            vec!["D".to_string()]
        );
    }
}
