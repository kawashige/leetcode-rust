pub struct Solution {}

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = (0..grid[0].len() as i32).collect::<Vec<i32>>();

        for row in grid {
            let mut state = vec![0; row.len()];
            for i in 0..row.len() {
                if (row[i] == 1 && (i == row.len() - 1 || (i < row.len() - 1 && row[i + 1] == -1)))
                    || (row[i] == -1 && (i == 0 || (0 < i && row[i - 1] == 1)))
                {
                    continue;
                }
                state[i] = row[i];
            }

            for i in 0..result.len() {
                if result[i] != -1 && state[result[i] as usize] != 0 {
                    result[i] += state[result[i] as usize];
                } else {
                    result[i] = -1;
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
    fn test_1706() {
        assert_eq!(
            Solution::find_ball(vec![
                vec![1, 1, 1, -1, -1],
                vec![1, 1, 1, -1, -1],
                vec![-1, -1, -1, 1, 1],
                vec![1, 1, 1, 1, -1],
                vec![-1, -1, -1, -1, -1]
            ]),
            vec![1, -1, -1, -1, -1]
        );
        assert_eq!(Solution::find_ball(vec![vec![-1]]), vec![-1]);
        assert_eq!(
            Solution::find_ball(vec![
                vec![1, 1, 1, 1, 1, 1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![1, 1, 1, 1, 1, 1],
                vec![-1, -1, -1, -1, -1, -1]
            ]),
            vec![0, 1, 2, 3, 4, -1]
        );
    }
}
