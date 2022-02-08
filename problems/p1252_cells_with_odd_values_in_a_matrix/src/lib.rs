pub struct Solution {}

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let (row, column) = indices.iter().fold(
            (vec![0; m as usize], vec![0; n as usize]),
            |(mut row, mut column), index| {
                row[index[0] as usize] += 1;
                column[index[1] as usize] += 1;
                (row, column)
            },
        );

        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if (row[i as usize] + column[j as usize]) % 2 == 1 {
                    result += 1;
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
    fn test_1252() {
        assert_eq!(
            Solution::odd_cells(
                28,
                38,
                vec![
                    vec![17, 16],
                    vec![26, 31],
                    vec![19, 12],
                    vec![22, 24],
                    vec![17, 28],
                    vec![23, 21],
                    vec![27, 32],
                    vec![23, 27],
                    vec![23, 33],
                    vec![18, 7],
                    vec![4, 20],
                    vec![0, 31],
                    vec![25, 33],
                    vec![5, 22]
                ]
            ),
            460
        );
        assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
        assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
    }
}
