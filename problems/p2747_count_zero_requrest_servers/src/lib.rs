use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let mut logs = logs;
        logs.sort_unstable_by_key(|log| log[1]);
        let mut queries = queries.into_iter().zip(0..).collect::<Vec<_>>();
        queries.sort_unstable();

        let mut count = vec![0; n as usize];
        let mut servers = 0;
        let mut queue = VecDeque::new();
        let mut result = vec![0; queries.len()];
        let mut j = 0;

        for i in 0..queries.len() {
            while j < logs.len() && logs[j][1] <= queries[i].0 {
                queue.push_back(logs[j].clone());
                count[logs[j][0] as usize - 1] += 1;
                if count[logs[j][0] as usize - 1] == 1 {
                    servers += 1;
                }
                j += 1;
            }
            while !queue.is_empty() && queue[0][1] < queries[i].0 - x {
                count[queue[0][0] as usize - 1] -= 1;
                if count[queue[0][0] as usize - 1] == 0 {
                    servers -= 1;
                }
                queue.pop_front();
            }
            result[queries[i].1] = n - servers;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2747() {
        assert_eq!(
            Solution::count_servers(3, vec![vec![1, 3], vec![2, 6], vec![1, 5]], 5, vec![10, 11]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::count_servers(
                3,
                vec![vec![2, 4], vec![2, 1], vec![1, 2], vec![3, 1]],
                2,
                vec![3, 4]
            ),
            vec![0, 1]
        );
    }
}
