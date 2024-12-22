use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut queries = queries
            .into_iter()
            .map(|v| vec![v[0].min(v[1]), v[0].max(v[1])])
            .zip(0..)
            .collect::<Vec<_>>();
        queries.sort_unstable_by(|a, b| a.0[0].cmp(&b.0[0]));

        let mut result = vec![-1; queries.len()];
        let mut heap = BinaryHeap::new();
        let mut heap2 = BinaryHeap::new();
        let mut j = 0;

        for i in 0..heights.len() {
            while j < queries.len() && queries[j].0[0] == i as i32 {
                if queries[j].0[0] == queries[j].0[1] {
                    result[queries[j].1] = queries[j].0[0];
                } else {
                    heap.push((Reverse(queries[j].0[1] as usize), i, queries[j].1));
                }
                j += 1;
            }
            while let Some((Reverse(h), k)) = heap2.pop() {
                if heights[i] < h {
                    heap2.push((Reverse(h), k));
                    break;
                }
                result[k] = i as i32;
            }
            while let Some((Reverse(b), a, k)) = heap.pop() {
                if i < b {
                    heap.push((Reverse(b), a, k));
                    break;
                }
                if heights[a] < heights[b] {
                    result[k] = b as i32;
                } else {
                    heap2.push((Reverse(heights[a].max(heights[b]) + 1), k));
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
    fn test_2940() {
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![6, 4, 8, 5, 2, 7],
                vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]
            ),
            vec![2, 5, -1, 5, 2]
        );
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![5, 3, 8, 2, 6, 1, 4, 6],
                vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]]
            ),
            vec![7, 6, -1, 4, 6]
        );
    }
}
