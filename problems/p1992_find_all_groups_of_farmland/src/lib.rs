pub struct Solution {}

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 1
                    && (i == 0 || land[i - 1][j] == 0)
                    && (j == 0 || land[i][j - 1] == 0)
                {
                    let (mut r, mut c) = (i, j);
                    while r < land.len() - 1 && land[r + 1][c] == 1 {
                        r += 1;
                    }
                    while c < land[0].len() - 1 && land[r][c + 1] == 1 {
                        c += 1;
                    }

                    result.push(vec![i as i32, j as i32, r as i32, c as i32]);
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
    fn test_1992() {
        assert_eq!(
            Solution::find_farmland(vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]]),
            vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]]
        );
        assert_eq!(
            Solution::find_farmland(vec![vec![1, 1], vec![1, 1]]),
            vec![vec![0, 0, 1, 1]]
        );
        assert_eq!(
            Solution::find_farmland(vec![vec![0]]),
            vec![] as Vec<Vec<i32>>
        );
    }
}
