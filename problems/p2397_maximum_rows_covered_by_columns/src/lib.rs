pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        columns: usize,
        count: i32,
        rows: &[usize],
        num_select: i32,
        result: &mut i32,
    ) {
        if num_select < columns.count_ones() as i32 {
            return;
        }
        *result = std::cmp::max(*result, count);

        for j in i..rows.len() {
            Self::recurse(
                j + 1,
                columns | rows[j],
                count + 1,
                rows,
                num_select,
                result,
            );
        }
    }

    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let rows = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .fold(0, |acc, (i, v)| if v == &1 { acc | 1 << i } else { acc })
            })
            .collect::<Vec<_>>();

        let mut result = 0;
        Self::recurse(0, 0, 0, &rows, num_select, &mut result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2397() {
        assert_eq!(
            Solution::maximum_rows(vec![vec![1, 0], vec![0, 1], vec![1, 1]], 1),
            1
        );
        assert_eq!(
            Solution::maximum_rows(
                vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]],
                2
            ),
            3
        );
        assert_eq!(Solution::maximum_rows(vec![vec![1], vec![0]], 1), 2);
    }
}
