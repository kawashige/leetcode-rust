use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];
        let mut queries = queries.into_iter().zip(0..).collect::<Vec<_>>();
        queries.sort_unstable();
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(grid[0][0]), (0, 0)));
        let mut i = 0;
        let mut points = 0;
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

        while let Some((Reverse(v), (r, c))) = heap.pop() {
            if seen[r][c] {
                continue;
            }
            seen[r][c] = true;
            while i < queries.len() && queries[i].0 <= v {
                result[queries[i].1] = points;
                i += 1;
            }
            if queries.len() == i {
                break;
            }
            points += 1;
            for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_r, new_c) = (r as i32 + dr, c as i32 + dc);
                if !(0..grid.len() as i32).contains(&new_r)
                    || !(0..grid[0].len() as i32).contains(&new_c)
                    || seen[new_r as usize][new_c as usize]
                {
                    continue;
                }
                heap.push((
                    Reverse(grid[new_r as usize][new_c as usize]),
                    (new_r as usize, new_c as usize),
                ));
            }
        }
        for j in i..queries.len() {
            result[queries[j].1] = points;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2503() {
        assert_eq!(
            Solution::max_points(
                vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
                vec![5, 6, 2]
            ),
            vec![5, 8, 1]
        );
        assert_eq!(
            Solution::max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3]),
            vec![0]
        );
    }
}
