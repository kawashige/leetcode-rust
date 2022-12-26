use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks = tasks.into_iter().enumerate().collect::<Vec<_>>();
        tasks.sort_unstable_by(|a, b| a.1[0].cmp(&b.1[0]));

        let mut time = 0;
        let mut heap = BinaryHeap::new();
        let mut i = 0;
        let mut result = Vec::new();

        while result.len() < tasks.len() {
            if heap.is_empty() && i < tasks.len() && time < tasks[i].1[0] {
                time = tasks[i].1[0];
            }
            while i < tasks.len() && tasks[i].1[0] <= time {
                heap.push((Reverse(tasks[i].1[1]), Reverse(tasks[i].0)));
                i += 1;
            }
            if let Some((Reverse(p), Reverse(j))) = heap.pop() {
                result.push(j as i32);
                time += p;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1834() {
        assert_eq!(
            Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]),
            vec![0, 2, 3, 1]
        );
        assert_eq!(
            Solution::get_order(vec![
                vec![7, 10],
                vec![7, 12],
                vec![7, 5],
                vec![7, 4],
                vec![7, 2]
            ]),
            vec![4, 3, 2, 0, 1]
        );
    }
}
