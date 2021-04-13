use std::collections::VecDeque;
pub struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut lists = vec![vec![]; n as usize];
        for v in flights {
            lists[v[0] as usize].push((v[1] as usize, v[2]));
        }

        let mut queue = VecDeque::new();
        queue.push_back((src as usize, 0, k + 1));

        let mut min = std::i32::MAX;
        let mut dist = vec![std::i32::MAX; n as usize];
        dist[src as usize] = 0;

        while let Some((node, cost, remains)) = queue.pop_front() {
            if node == dst as usize {
                min = std::cmp::min(min, cost);
                continue;
            }

            if remains == 0 {
                continue;
            }

            for (next, c) in &lists[node] {
                if cost + c < dist[*next] {
                    dist[*next] = cost + c;
                    queue.push_back((*next, cost + c, remains - 1));
                }
            }
        }

        if min == std::i32::MAX {
            -1
        } else {
            min
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0787() {
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            ),
            200
        );
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            ),
            500
        );
        assert_eq!(
            Solution::find_cheapest_price(
                10,
                vec![
                    vec![3, 4, 4],
                    vec![2, 5, 6],
                    vec![4, 7, 10],
                    vec![9, 6, 5],
                    vec![7, 4, 4],
                    vec![6, 2, 10],
                    vec![6, 8, 6],
                    vec![7, 9, 4],
                    vec![1, 5, 4],
                    vec![1, 0, 4],
                    vec![9, 7, 3],
                    vec![7, 0, 5],
                    vec![6, 5, 8],
                    vec![1, 7, 6],
                    vec![4, 0, 9],
                    vec![5, 9, 1],
                    vec![8, 7, 3],
                    vec![1, 2, 6],
                    vec![4, 1, 5],
                    vec![5, 2, 4],
                    vec![1, 9, 1],
                    vec![7, 8, 10],
                    vec![0, 4, 2],
                    vec![7, 2, 8]
                ],
                6,
                0,
                7
            ),
            14
        );
    }
}
