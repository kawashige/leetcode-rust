use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn is_ok(list: &Vec<Vec<(usize, i32)>>, weight_limit: i32) -> bool {
        let mut seen = vec![false; list.len() as usize];
        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut count = 0;

        while let Some(n) = queue.pop_front() {
            if seen[n] {
                continue;
            }
            count += 1;
            seen[n] = true;

            for (next_node, w) in &list[n] {
                if w <= &weight_limit {
                    queue.push_back(*next_node);
                }
            }
        }

        count == list.len()
    }

    pub fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let mut list = vec![vec![]; n as usize];
        for e in edges {
            list[e[1] as usize].push((e[0] as usize, e[2]));
        }

        if !Self::is_ok(&list, 1_000_000) {
            return -1;
        }

        let mut ok = 1_000_000;
        let mut ng = 0;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(&list, mid) {
                ok = mid;
            } else {
                ng = mid
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3419() {
        assert_eq!(
            Solution::min_max_weight(
                5,
                vec![
                    vec![1, 0, 1],
                    vec![2, 0, 2],
                    vec![3, 0, 1],
                    vec![4, 3, 1],
                    vec![2, 1, 1]
                ],
                2
            ),
            1
        );
        assert_eq!(
            Solution::min_max_weight(
                5,
                vec![
                    vec![0, 1, 1],
                    vec![0, 2, 2],
                    vec![0, 3, 1],
                    vec![0, 4, 1],
                    vec![1, 2, 1],
                    vec![1, 4, 1]
                ],
                1
            ),
            -1
        );
        assert_eq!(
            Solution::min_max_weight(
                5,
                vec![
                    vec![1, 2, 1],
                    vec![1, 3, 3],
                    vec![1, 4, 5],
                    vec![2, 3, 2],
                    vec![3, 4, 2],
                    vec![4, 0, 1]
                ],
                1
            ),
            2
        );
        assert_eq!(
            Solution::min_max_weight(
                5,
                vec![
                    vec![1, 2, 1],
                    vec![1, 3, 3],
                    vec![1, 4, 5],
                    vec![2, 3, 2],
                    vec![4, 0, 1]
                ],
                1
            ),
            -1
        );
    }
}
