pub struct Solution {}

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut diagonal_group: Vec<Vec<i32>> = vec![vec![]; mat.len() + mat[0].len() - 1];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                diagonal_group[mat[0].len() - 1 + i - j].push(mat[i][j]);
            }
        }
        for i in 0..diagonal_group.len() {
            diagonal_group[i].sort_unstable();
        }
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                let k = if i < j { i } else { j };
                mat[i][j] = diagonal_group[mat[0].len() - 1 + i - j][k];
            }
        }

        mat
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day23() {
        assert_eq!(
            Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]),
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
        );
    }
}
