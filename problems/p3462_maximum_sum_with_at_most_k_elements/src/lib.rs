use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                heap.push((grid[i][j], i));
            }
        }
        let mut limits = limits;
        let mut result = 0;
        let mut k = k;
        while 0 < k {
            if let Some((x, i)) = heap.pop() {
                if limits[i] == 0 {
                    continue;
                }
                k -= 1;
                limits[i] -= 1;
                result += x as i64;
            } else {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3462() {
        assert_eq!(
            Solution::max_sum(
                vec![vec![7, 10, 3, 3, 7, 7, 0], vec![5, 5, 9, 2, 10, 5, 2]],
                vec![3, 7],
                7
            ),
            53
        );
        assert_eq!(
            Solution::max_sum(vec![vec![1, 2], vec![3, 4]], vec![1, 2], 2),
            7
        );
        assert_eq!(
            Solution::max_sum(vec![vec![5, 3, 7], vec![8, 2, 6]], vec![2, 2], 3),
            21
        );
    }
}
