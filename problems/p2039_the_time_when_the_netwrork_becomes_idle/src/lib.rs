use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let mut list = vec![vec![]; patience.len()];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }
        let mut dist = vec![-1; patience.len()];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while let Some(i) = queue.pop_front() {
            if dist[i.0] != -1 {
                continue;
            }
            dist[i.0] = i.1;

            for next in &list[i.0] {
                if dist[i.0] != -1 {
                    queue.push_back((*next, i.1 + 1));
                }
            }
        }

        let mut time = 0;

        for i in 1..dist.len() {
            let tmp_time = ((dist[i] * 2 - 1) / patience[i]) * patience[i] + dist[i] * 2 + 1;
            time = time.max(tmp_time);
        }

        time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2039() {
        assert_eq!(
            Solution::network_becomes_idle(
                vec![
                    vec![5, 7],
                    vec![15, 18],
                    vec![12, 6],
                    vec![5, 1],
                    vec![11, 17],
                    vec![3, 9],
                    vec![6, 11],
                    vec![14, 7],
                    vec![19, 13],
                    vec![13, 3],
                    vec![4, 12],
                    vec![9, 15],
                    vec![2, 10],
                    vec![18, 4],
                    vec![5, 14],
                    vec![17, 5],
                    vec![16, 2],
                    vec![7, 1],
                    vec![0, 16],
                    vec![10, 19],
                    vec![1, 8]
                ],
                vec![0, 2, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1]
            ),
            67
        );
        assert_eq!(
            Solution::network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1]),
            8
        );
        assert_eq!(
            Solution::network_becomes_idle(
                vec![vec![0, 1], vec![0, 2], vec![1, 2]],
                vec![0, 10, 10]
            ),
            3
        );
    }
}
