use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut connected = vec![false; points.len()];
        heap.push((Reverse(0), 0));
        let mut min_cost = 0;

        while let Some((Reverse(cost), i)) = heap.pop() {
            if connected[i] {
                continue;
            }
            connected[i] = true;
            min_cost += cost;

            for j in 0..points.len() {
                if !connected[j] {
                    let c =
                        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                    heap.push((Reverse(c), j));
                }
            }
        }

        min_cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1584() {
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![0, 0],
                vec![2, 2],
                vec![3, 10],
                vec![5, 2],
                vec![7, 0]
            ]),
            20
        );
        assert_eq!(
            Solution::min_cost_connect_points(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]),
            18
        );
    }
}
