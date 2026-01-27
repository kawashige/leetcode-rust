use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut seen = vec![std::i32::MAX; n];
        let mut list = vec![vec![]; n];
        let mut list2 = vec![vec![]; n];
        for i in 0..edges.len() {
            list[edges[i][0] as usize].push((edges[i][1] as usize, edges[i][2]));
            list2[edges[i][1] as usize].push((edges[i][0] as usize, edges[i][2] * 2));
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while let Some((i, c)) = queue.pop_front() {
            if seen[i] <= c {
                continue;
            }
            seen[i] = c;
            for (next, cost) in &list[i] {
                if seen[*next] <= c + cost {
                    continue;
                }
                queue.push_back((*next, c + cost));
            }
            for (next, cost) in &list2[i] {
                if seen[*next] <= c + cost {
                    continue;
                }
                queue.push_back((*next, c + cost));
            }
        }

        if seen[n as usize - 1] == std::i32::MAX {
            -1
        } else {
            seen[n as usize - 1]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3650() {
        assert_eq!(
            Solution::min_cost(
                4,
                vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]]
            ),
            5
        );
        assert_eq!(
            Solution::min_cost(
                4,
                vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]]
            ),
            3
        );
    }
}
