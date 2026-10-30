use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let mut list = vec![vec![]; n as usize];
        for e in edges {
            list[e[0] as usize].push((e[1] as usize, e[2]));
        }

        let mut min_weights = vec![vec![std::i32::MAX; k as usize + 1]; n as usize];
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), Reverse(1), 0));

        while let Some((Reverse(weight), Reverse(count), cur)) = heap.pop() {
            for (next, w) in &list[cur] {
                let next_count = if labels.as_bytes()[cur] != labels.as_bytes()[*next] {
                    1
                } else {
                    count + 1
                };
                if next_count <= k as usize && w + weight < min_weights[*next][next_count] {
                    min_weights[*next][next_count] = weight + w;
                    heap.push((Reverse(weight + w), Reverse(next_count), *next));
                }
            }
        }

        let result = *min_weights[n as usize - 1].iter().min().unwrap();
        if result == std::i32::MAX { -1 } else { result }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3970() {
        assert_eq!(
            Solution::shortest_path(
                3,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]],
                "aab".to_string(),
                1
            ),
            3
        );
        assert_eq!(
            Solution::shortest_path(
                3,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]],
                "aab".to_string(),
                2
            ),
            2
        );
        assert_eq!(
            Solution::shortest_path(3, vec![vec![0, 1, 1], vec![1, 2, 1]], "aaa".to_string(), 2),
            -1
        );
    }
}

fn main() {}
