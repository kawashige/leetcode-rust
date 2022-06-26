use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Solution {}

#[derive(PartialEq, Debug)]
struct MinNonNan(f64);

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for MinNonNan {}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut list = vec![vec![]; n as usize];

        for i in 0..edges.len() {
            list[edges[i][0] as usize].push((edges[i][1] as usize, succ_prob[i]));
            list[edges[i][1] as usize].push((edges[i][0] as usize, succ_prob[i]));
        }

        let mut prob = vec![0.0; n as usize];
        prob[start as usize] = 1.0;

        let mut heap = BinaryHeap::new();
        heap.push((MinNonNan(1.0), start as usize));
        while let Some((MinNonNan(p), i)) = heap.pop() {
            if p < prob[i] {
                continue;
            }
            if i == end as usize {
                return p;
            }

            for (next, next_p) in &list[i] {
                if prob[*next] < next_p * p {
                    prob[*next] = next_p * p;
                    heap.push((MinNonNan(prob[*next]), *next));
                }
            }
        }

        0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1514() {
        assert_eq!(
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.2],
                0,
                2
            ),
            0.25000
        );
        assert_eq!(
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.3],
                0,
                2
            ),
            0.30000
        );
        assert_eq!(
            Solution::max_probability(3, vec![vec![0, 1]], vec![0.5], 0, 2),
            0.00000
        );
    }
}
