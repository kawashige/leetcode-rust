use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![-1; n as usize];
        let mut list = vec![vec![]; n as usize];
        for edge in edges {
            list[edge[0] as usize].push((edge[1] as usize, edge[2]));
            list[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0));
        while let Some((Reverse(t), n)) = heap.pop() {
            if answer[n] != -1 {
                continue;
            }
            answer[n] = t;
            for (next, d) in &list[n] {
                if disappear[*next] <= t + d {
                    continue;
                }
                heap.push((Reverse(t + d), *next));
            }
        }

        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3112() {
        assert_eq!(
            Solution::minimum_time(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
                vec![1, 1, 5]
            ),
            vec![0, -1, 4]
        );
        assert_eq!(
            Solution::minimum_time(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
                vec![1, 3, 5]
            ),
            vec![0, 2, 3]
        );
        assert_eq!(
            Solution::minimum_time(2, vec![vec![0, 1, 1]], vec![1, 1]),
            vec![0, -1]
        );
    }
}
