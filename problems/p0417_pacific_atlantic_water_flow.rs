pub struct Solution {}

impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn recurse(matrix: &Vec<Vec<i32>>, start: Vec<Vec<usize>>, result: &mut Vec<Vec<usize>>) {
            let mut current = start;
            while !current.is_empty() {
                let mut next = Vec::new();
                for c in current {
                    result[c[0]][c[1]] = 1;
                    if 0 < c[0]
                        && matrix[c[0]][c[1]] <= matrix[c[0] - 1][c[1]]
                        && result[c[0] - 1][c[1]] == 0
                    {
                        next.push(vec![c[0] - 1, c[1]]);
                    }
                    if 0 < c[1]
                        && matrix[c[0]][c[1]] <= matrix[c[0]][c[1] - 1]
                        && result[c[0]][c[1] - 1] == 0
                    {
                        next.push(vec![c[0], c[1] - 1]);
                    }
                    if c[0] < matrix.len() - 1
                        && matrix[c[0]][c[1]] <= matrix[c[0] + 1][c[1]]
                        && result[c[0] + 1][c[1]] == 0
                    {
                        next.push(vec![c[0] + 1, c[1]]);
                    }
                    if c[1] < matrix[0].len() - 1
                        && matrix[c[0]][c[1]] <= matrix[c[0]][c[1] + 1]
                        && result[c[0]][c[1] + 1] == 0
                    {
                        next.push(vec![c[0], c[1] + 1]);
                    }
                }
                current = next;
            }
        }

        let mut results = Vec::new();

        if matrix.is_empty() {
            return results;
        }

        let mut pacific = vec![vec![0; matrix[0].len()]; matrix.len()];
        let current = (0..matrix[0].len())
            .map(|i| vec![0, i])
            .chain((0..matrix.len()).map(|i| vec![i, 0]))
            .collect::<Vec<Vec<usize>>>();
        recurse(&matrix, current, &mut pacific);

        let mut atlantic = vec![vec![0; matrix[0].len()]; matrix.len()];
        let current = (0..matrix[0].len())
            .map(|i| vec![matrix.len() - 1, i])
            .chain((0..matrix.len()).map(|i| vec![i, matrix[0].len() - 1]))
            .collect::<Vec<Vec<usize>>>();
        recurse(&matrix, current, &mut atlantic);

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if pacific[i][j] == 1 && atlantic[i][j] == 1 {
                    results.push(vec![i as i32, j as i32]);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0417() {
        assert_eq!(
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ],
            Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4],
            ])
        );
        assert_eq!(
            Vec::new() as Vec<Vec<i32>>,
            Solution::pacific_atlantic(Vec::new())
        );
        assert_eq!(vec![vec![0, 0]], Solution::pacific_atlantic(vec![vec![9]]));
    }
}
