use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut list = vec![vec![]; n as usize + 1];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }
        let mut result = 0;
        let mut seen = vec![false; n as usize + 1];

        for i in 1..seen.len() {
            if seen[i] {
                continue;
            }
            let mut stack = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back(i);
            while let Some(node) = queue.pop_front() {
                if seen[node] {
                    continue;
                }
                seen[node] = true;
                stack.push(node);
                for next in &list[node] {
                    if seen[*next] {
                        continue;
                    }
                    queue.push_back(*next);
                }
            }
            let mut max_depth1 = 0;

            while let Some(node) = stack.pop() {
                let mut max_depth = 0;
                let mut depth = vec![0; n as usize + 1];
                let mut queue = VecDeque::new();
                queue.push_back((node, 1));
                while let Some((j, d)) = queue.pop_front() {
                    if depth[j] != 0 {
                        if depth[j] != d {
                            return -1;
                        }
                        continue;
                    }
                    depth[j] = d;
                    max_depth = max_depth.max(d);
                    for next in &list[j] {
                        if depth[*next] != 0 {
                            continue;
                        }
                        queue.push_back((*next, d + 1));
                    }
                }
                max_depth1 = max_depth1.max(max_depth);
            }

            result += max_depth1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2493() {
        assert_eq!(
            Solution::magnificent_sets(
                6,
                vec![
                    vec![1, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 6],
                    vec![2, 3],
                    vec![4, 6]
                ]
            ),
            4
        );
        assert_eq!(
            Solution::magnificent_sets(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]),
            -1
        );
    }
}
