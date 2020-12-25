pub struct Solution {}

impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return Vec::new();
        }
        let mut result = vec![matrix[0][0]];
        let mut current = [0, 0];
        let mut up_phase = true;
        while result.len() < matrix.len() * matrix[0].len() {
            if up_phase {
                if current[1] == matrix[0].len() - 1 {
                    current[0] += 1;
                    up_phase = false;
                } else if current[0] == 0 {
                    current[1] += 1;
                    up_phase = false;
                } else {
                    current[0] = (current[0] as i32 - 1) as usize;
                    current[1] = (current[1] as i32 + 1) as usize;
                }
            } else {
                if current[0] == matrix.len() - 1 {
                    current[1] += 1;
                    up_phase = true;
                } else if current[1] == 0 {
                    current[0] += 1;
                    up_phase = true;
                } else {
                    current[0] = (current[0] as i32 + 1) as usize;
                    current[1] = (current[1] as i32 - 1) as usize;
                }
            }
            result.push(matrix[current[0]][current[1]]);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day25() {
        assert_eq!(
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9],
            Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
        assert_eq!(
            vec![1, 2, 3],
            Solution::find_diagonal_order(vec![vec![1, 2, 3]])
        );
        assert_eq!(
            vec![1, 2, 3],
            Solution::find_diagonal_order(vec![vec![1], vec![2], vec![3]])
        );
        assert_eq!(vec![] as Vec<i32>, Solution::find_diagonal_order(vec![]));
        assert_eq!(vec![1], Solution::find_diagonal_order(vec![vec![1]]));
    }
}
