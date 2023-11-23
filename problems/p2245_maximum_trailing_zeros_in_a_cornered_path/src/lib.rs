pub struct Solution {}

impl Solution {
    pub fn trailing_zeros(count1: &[i32], count2: &[i32], count3: &[i32]) -> i32 {
        count1[0]
            + count2[0]
            + count3[0]
            + (count1[1] + count2[1] + count3[1]).min(count1[2] + count2[2] + count3[2])
    }

    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![vec![vec![0; 3]; grid[0].len()]; grid.len()];
        let mut left = vec![vec![vec![0; 3]; grid[0].len() + 1]; grid.len() + 1];
        let mut up = vec![vec![vec![0; 3]; grid[0].len() + 1]; grid.len() + 1];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut num = grid[i][j];
                while num % 10 == 0 {
                    count[i][j][0] += 1;
                    num /= 10;
                }
                while num % 5 == 0 {
                    count[i][j][2] += 1;
                    num /= 5;
                }
                while num % 2 == 0 {
                    count[i][j][1] += 1;
                    num /= 2;
                }
                for k in 0..3 {
                    up[i + 1][j + 1][k] = count[i][j][k];
                    left[i + 1][j + 1][k] = count[i][j][k];
                    if 0 < i {
                        up[i + 1][j + 1][k] += up[i][j + 1][k];
                    }
                    if 0 < j {
                        left[i + 1][j + 1][k] += left[i + 1][j][k];
                    }
                }
            }
        }

        let mut max = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let left_count = &left[i + 1][j];
                let up_count = &up[i][j + 1];
                let right_count = (0..3)
                    .map(|k| left[i + 1][grid[0].len()][k] - left[i + 1][j + 1][k])
                    .collect::<Vec<_>>();
                let down_count = (0..3)
                    .map(|k| up[grid.len()][j + 1][k] - up[i + 1][j + 1][k])
                    .collect::<Vec<_>>();

                max = (Self::trailing_zeros(&count[i][j], left_count, up_count)).max(max);
                max = (Self::trailing_zeros(&count[i][j], up_count, &right_count)).max(max);
                max = (Self::trailing_zeros(&count[i][j], left_count, &down_count)).max(max);
                max = (Self::trailing_zeros(&count[i][j], &down_count, &right_count)).max(max);
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2245() {
        assert_eq!(
            Solution::max_trailing_zeros(vec![
                vec![23, 17, 15, 3, 20],
                vec![8, 1, 20, 27, 11],
                vec![9, 4, 6, 2, 21],
                vec![40, 9, 1, 10, 6],
                vec![22, 7, 4, 5, 3]
            ]),
            3
        );
        assert_eq!(
            Solution::max_trailing_zeros(vec![vec![4, 3, 2], vec![7, 6, 1], vec![8, 8, 8]]),
            0
        );
        assert_eq!(Solution::max_trailing_zeros(vec![vec![1, 5, 2, 4, 25]]), 3);
    }
}
