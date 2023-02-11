use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut used = BinaryHeap::new();
        let mut free = BinaryHeap::new();
        for i in 0..servers.len() {
            free.push((Reverse(servers[i]), Reverse(i)));
        }

        let mut result = Vec::with_capacity(tasks.len());
        for i in 0..tasks.len() {
            while let Some((Reverse(time), _, Reverse(server))) = used.pop() {
                if i < time {
                    used.push((Reverse(time), Reverse(servers[server]), Reverse(server)));
                    break;
                }
                free.push((Reverse(servers[server]), Reverse(server)));
            }

            if let Some((_, Reverse(server))) = free.pop() {
                result.push(server as i32);
                used.push((
                    Reverse(i + tasks[i] as usize),
                    Reverse(servers[server]),
                    Reverse(server),
                ));
            } else {
                if let Some((Reverse(time), _, Reverse(server))) = used.pop() {
                    result.push(server as i32);
                    used.push((
                        Reverse(time + tasks[i] as usize),
                        Reverse(servers[server]),
                        Reverse(server),
                    ));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1882() {
        assert_eq!(
            Solution::assign_tasks(
                vec![31, 96, 73, 90, 15, 11, 1, 90, 72, 9, 30, 88],
                vec![87, 10, 3, 5, 76, 74, 38, 64, 16, 64, 93, 95, 60, 79, 54, 26, 30, 44, 64, 71]
            ),
            vec![6, 9, 5, 4, 10, 5, 0, 8, 4, 2, 11, 9, 3, 7, 1, 4, 0, 4, 1, 8]
        );
        assert_eq!(
            Solution::assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]),
            vec![2, 2, 0, 2, 1, 2]
        );
        assert_eq!(
            Solution::assign_tasks(vec![5, 1, 4, 3, 2], vec![2, 1, 2, 4, 5, 2, 1]),
            vec![1, 4, 1, 4, 1, 3, 2]
        );
    }
}
