pub struct Solution {}

impl Solution {
    pub fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
        let mut values = vec![vec![vec![]; image[0].len()]; image.len()];
        for i in 0..image.len() - 2 {
            for j in 0..image[0].len() - 2 {
                let mut sum = 0;
                let mut is_valid = true;

                for di in 0..3 {
                    for dj in 0..3 {
                        if !((di == 0
                            || (image[i + di - 1][j + dj] - image[i + di][j + dj]).abs()
                                <= threshold)
                            && (dj == 0
                                || (image[i + di][j + dj - 1] - image[i + di][j + dj]).abs()
                                    <= threshold))
                        {
                            is_valid = false;
                            break;
                        }
                        sum += image[i + di][j + dj];
                    }
                    if !is_valid {
                        break;
                    }
                }

                if is_valid {
                    for di in 0..3 {
                        for dj in 0..3 {
                            values[i + di][j + dj].push(sum / 9);
                        }
                    }
                }
            }
        }

        let mut result = vec![vec![0; image[0].len()]; image.len()];
        for i in 0..result.len() {
            for j in 0..result[0].len() {
                if values[i][j].is_empty() {
                    result[i][j] = image[i][j];
                } else {
                    result[i][j] = values[i][j].iter().sum::<i32>() / values[i][j].len() as i32;
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
    fn test_3030() {
        assert_eq!(
            Solution::result_grid(
                vec![vec![5, 6, 7, 10], vec![8, 9, 10, 10], vec![11, 12, 13, 10]],
                3
            ),
            vec![vec![9, 9, 9, 9], vec![9, 9, 9, 9], vec![9, 9, 9, 9]]
        );
        assert_eq!(
            Solution::result_grid(
                vec![
                    vec![10, 20, 30],
                    vec![15, 25, 35],
                    vec![20, 30, 40],
                    vec![25, 35, 45]
                ],
                12
            ),
            vec![
                vec![25, 25, 25],
                vec![27, 27, 27],
                vec![27, 27, 27],
                vec![30, 30, 30]
            ]
        );
        assert_eq!(
            Solution::result_grid(vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]], 1),
            vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]]
        );
    }
}
