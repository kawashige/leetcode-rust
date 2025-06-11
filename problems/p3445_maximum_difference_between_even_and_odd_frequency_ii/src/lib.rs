use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut result = std::i32::MIN;

        for x in 0..5 {
            for y in 0..5 {
                if x == y {
                    continue;
                }

                let mut min_value = vec![vec![1_000_000; 2]; 2];
                let mut counts = vec![[0; 2]; s.len()];
                let mut queue = VecDeque::new();

                for i in 0..s.len() {
                    if 0 < i {
                        counts[i][0] = counts[i - 1][0];
                        counts[i][1] = counts[i - 1][1];
                    }
                    if (s.as_bytes()[i] - b'0') as usize == x {
                        counts[i][0] += 1;
                    } else if (s.as_bytes()[i] - b'0') as usize == y {
                        counts[i][1] += 1;
                    }
                    queue.push_back((counts[i][0], counts[i][1]));

                    while k < queue.len()
                        && 0 < counts[i][0] - queue[0].0
                        && 1 < counts[i][1] - queue[0].1
                    {
                        min_value[(queue[0].0 % 2) as usize][(queue[0].1 % 2) as usize] = min_value
                            [(queue[0].0 % 2) as usize][(queue[0].1 % 2) as usize]
                            .min(queue[0].0 - queue[0].1);
                        queue.pop_front();
                    }

                    if k <= i + 1 && 0 < counts[i][0] && 1 < counts[i][1] {
                        if counts[i][0] % 2 == 1 && counts[i][1] % 2 == 0 {
                            result = result.max(counts[i][0] - counts[i][1]);
                        }
                        result = result.max(
                            counts[i][0]
                                - counts[i][1]
                                - min_value[(((counts[i][0] % 2) + 1) % 2) as usize]
                                    [(counts[i][1] % 2) as usize],
                        );
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
    fn test_3445() {
        assert_eq!(Solution::max_difference("44114402".to_string(), 7), 1);
        assert_eq!(Solution::max_difference("12233".to_string(), 4), -1);
        assert_eq!(Solution::max_difference("1122211".to_string(), 3), 1);
        assert_eq!(Solution::max_difference("110".to_string(), 3), -1);
    }
}
