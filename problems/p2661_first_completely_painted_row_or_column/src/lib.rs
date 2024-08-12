use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut row = vec![0; mat.len()];
        let mut column = vec![0; mat[0].len()];
        let map = arr.into_iter().zip(0..).collect::<HashMap<_, _>>();

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                row[i] = row[i].max(map[&mat[i][j]]);
                column[j] = column[j].max(map[&mat[i][j]]);
            }
        }

        row.into_iter().chain(column.into_iter()).min().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2661() {
        assert_eq!(
            Solution::first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]),
            2
        );
        assert_eq!(
            Solution::first_complete_index(
                vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
                vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]]
            ),
            3
        );
    }
}
