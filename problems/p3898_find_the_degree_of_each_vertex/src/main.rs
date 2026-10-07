pub struct Solution {}

impl Solution {
    pub fn find_degrees(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..i {
                if matrix[i][j] == 1 {
                    result[i] += 1;
                    result[j] += 1;
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
    fn test_3898() {
        assert_eq!(
            Solution::find_degrees(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]),
            vec![2, 2, 2]
        );
        assert_eq!(
            Solution::find_degrees(vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 0]]),
            vec![1, 1, 0]
        );
        assert_eq!(Solution::find_degrees(vec![vec![0]]), vec![0]);
    }
}

fn main() {}
