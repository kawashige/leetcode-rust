use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut result = 0;
        let mut max_points = 0;
        let mut heap = BinaryHeap::new();

        for i in 0..questions.len() {
            while let Some((Reverse(index), value)) = heap.pop() {
                if i <= index {
                    heap.push((Reverse(index), value));
                    break;
                }
                max_points = max_points.max(value);
            }
            result = result.max(max_points + questions[i][0] as i64);
            heap.push((
                Reverse(i + questions[i][1] as usize),
                max_points + questions[i][0] as i64,
            ));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2140() {
        assert_eq!(
            Solution::most_points(vec![
                vec![21, 5],
                vec![92, 3],
                vec![74, 2],
                vec![39, 4],
                vec![58, 2],
                vec![5, 5],
                vec![49, 4],
                vec![65, 3]
            ]),
            157
        );
        assert_eq!(
            Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
            5
        );
        assert_eq!(
            Solution::most_points(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5]
            ]),
            7
        );
    }
}
