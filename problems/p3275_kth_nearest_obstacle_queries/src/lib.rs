use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut result = Vec::with_capacity(queries.len());
        for q in queries {
            heap.push(q[0].abs() + q[1].abs());
            if heap.len() == k as usize + 1 {
                heap.pop();
            }
            result.push(if heap.len() != k as usize {
                -1
            } else {
                *heap.peek().unwrap()
            });
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3275() {
        assert_eq!(
            Solution::results_array(vec![vec![1, 2], vec![3, 4], vec![2, 3], vec![-3, 0]], 2),
            vec![-1, 7, 5, 3]
        );
        assert_eq!(
            Solution::results_array(vec![vec![5, 5], vec![4, 4], vec![3, 3]], 1),
            vec![10, 8, 6]
        );
    }
}
