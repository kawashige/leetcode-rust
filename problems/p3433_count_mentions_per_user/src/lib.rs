use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut is_online = vec![true; number_of_users as usize];
        let mut events = events
            .into_iter()
            .map(|e| {
                (
                    if &e[0] == "OFFLINE" { 0 } else { 1 },
                    e[1].parse::<usize>().unwrap(),
                    e[2].clone(),
                )
            })
            .collect::<Vec<_>>();
        events.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        let mut queue = VecDeque::new();
        let mut result = vec![0; number_of_users as usize];
        for e in events {
            while let Some((i, t)) = queue.pop_front() {
                if t <= e.1 {
                    is_online[i] = true;
                } else {
                    queue.push_front((i, t));
                    break;
                }
            }

            if e.0 == 0 {
                let id = e.2.parse::<usize>().unwrap();
                is_online[id] = false;
                queue.push_back((id, e.1 + 60));
            } else {
                match e.2.as_str() {
                    "ALL" => {
                        for i in 0..result.len() {
                            result[i] += 1;
                        }
                    }
                    "HERE" => {
                        for i in 0..result.len() {
                            if is_online[i] {
                                result[i] += 1;
                            }
                        }
                    }
                    _ => {
                        for i in
                            e.2.split_ascii_whitespace()
                                .map(|i| i.trim_start_matches("id").parse::<usize>().unwrap())
                        {
                            result[i] += 1;
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3433() {
        assert_eq!(
            Solution::count_mentions(
                2,
                vec![
                    vec![
                        "MESSAGE".to_string(),
                        "10".to_string(),
                        "id1 id0".to_string()
                    ],
                    vec!["OFFLINE".to_string(), "11".to_string(), "0".to_string()],
                    vec!["MESSAGE".to_string(), "71".to_string(), "HERE".to_string()]
                ]
            ),
            vec![2, 2]
        );
        assert_eq!(
            Solution::count_mentions(
                2,
                vec![
                    vec![
                        "MESSAGE".to_string(),
                        "10".to_string(),
                        "id1 id0".to_string()
                    ],
                    vec!["OFFLINE".to_string(), "11".to_string(), "0".to_string()],
                    vec!["MESSAGE".to_string(), "12".to_string(), "ALL".to_string()]
                ]
            ),
            vec![2, 2]
        );
        assert_eq!(
            Solution::count_mentions(
                2,
                vec![
                    vec!["OFFLINE".to_string(), "10".to_string(), "0".to_string()],
                    vec!["MESSAGE".to_string(), "12".to_string(), "HERE".to_string()]
                ]
            ),
            vec![0, 1]
        );
    }
}
