pub struct Solution {}

impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; m[0].len()]; m.len()];
        for i in 0..m.len() {
            for j in 0..m[0].len() {
                let mut n = 0;
                for (x, y) in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 0),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                {
                    let x = x + i as i32;
                    let y = y + j as i32;
                    if x < 0 || m.len() as i32 - 1 < x || y < 0 || m[0].len() as i32 - 1 < y {
                        continue;
                    }
                    n += 1;
                    result[i][j] += m[x as usize][y as usize];
                }
                if 0 < n {
                    result[i][j] /= n;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn test_0661() {
        assert_eq!(
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
    }
}
