use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let mut seen = vec![vec![]; n as usize + 1];
        let mut queue = VecDeque::new();
        queue.push_back((1, 0));

        let mut list = vec![vec![]; n as usize + 1];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        while let Some((v, t)) = queue.pop_front() {
            if v == n as usize && seen[v].len() == 1 && seen[v][0] < t {
                return t;
            }
            let leave_t = if (t / change) % 2 == 0 {
                t
            } else {
                change * (t / change + 1)
            };
            if seen[v].len() == 2 || (!seen[v].is_empty() && seen[v][0] == leave_t) {
                continue;
            }
            seen[v].push(leave_t);

            for next in &list[v] {
                queue.push_back((*next, leave_t + time));
            }
        }
        println!("seen: {:?}", seen);

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2645() {
        assert_eq!(
            Solution::second_minimum(
                6,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 4],
                    vec![3, 5],
                    vec![5, 4],
                    vec![4, 6]
                ],
                3,
                100
            ),
            12
        );
        // assert_eq!(
        //     Solution::second_minimum(
        //         5,
        //         vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
        //         3,
        //         5
        //     ),
        //     13
        // );
        // assert_eq!(Solution::second_minimum(2, vec![vec![1, 2]], 3, 2), 11);
    }
}
