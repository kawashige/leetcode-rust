pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..(matrix.len() / 2) {
            for j in i..(matrix.len() - 1 - i) {
                let mut current = (j, matrix.len() - 1 - i);
                let mut val = matrix[i][j];
                loop {
                    let tmp = matrix[current.0][current.1];
                    matrix[current.0][current.1] = val;
                    val = tmp;
                    if current == (i, j) {
                        break;
                    }
                    current = (current.1, matrix.len() - 1 - current.0);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0048() {
        let mut m1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let r1 = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        Solution::rotate(&mut m1);
        assert_eq!(r1, m1);

        let mut m2 = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let r2 = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        Solution::rotate(&mut m2);
        assert_eq!(r2, m2);

        let mut m3 = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        let r3 = vec![
            vec![21, 16, 11, 6, 1],
            vec![22, 17, 12, 7, 2],
            vec![23, 18, 13, 8, 3],
            vec![24, 19, 14, 9, 4],
            vec![25, 20, 15, 10, 5],
        ];
        Solution::rotate(&mut m3);
        assert_eq!(r3, m3);
    }
}
