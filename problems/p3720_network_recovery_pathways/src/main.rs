use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn check(list: &Vec<Vec<(usize, i64)>>, max_cost: i64, k: i64) -> bool {
        let mut dist = vec![std::i64::MAX; list.len()];
        dist[0] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0));
        while let Some((_, i)) = heap.pop() {
            if i == list.len() - 1 {
                return true;
            }
            for (j, d) in &list[i] {
                if &max_cost <= d && dist[i] + d < dist[*j].min(k + 1) {
                    dist[*j] = dist[i] + d;
                    heap.push((Reverse(dist[*j]), *j));
                }
            }
        }

        false
    }

    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let mut list = vec![vec![]; online.len()];
        let mut max_cost = 0;
        for e in edges {
            if online[e[0] as usize] && online[e[1] as usize] {
                list[e[0] as usize].push((e[1] as usize, e[2] as i64));
                max_cost = max_cost.max(e[2] as i64);
            }
        }

        if !Self::check(&list, 0, k) {
            return -1;
        }

        let mut ok = 0;
        let mut ng = max_cost + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::check(&list, mid, k) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3730() {
        assert_eq!(
            Solution::find_max_path_score(
                vec![vec![0, 1, 5], vec![1, 3, 10], vec![0, 2, 3], vec![2, 3, 4]],
                vec![true, true, true, true],
                10
            ),
            3
        );
        assert_eq!(
            Solution::find_max_path_score(
                vec![
                    vec![0, 1, 7],
                    vec![1, 4, 5],
                    vec![0, 2, 6],
                    vec![2, 3, 6],
                    vec![3, 4, 2],
                    vec![2, 4, 6]
                ],
                vec![true, true, true, false, true],
                12
            ),
            6
        );
    }
}

fn main() {}
