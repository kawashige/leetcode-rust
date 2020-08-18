pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        }
        let mut c0 = 0;
        let mut c1 = matrix.len() - 1;

        while c1 - c0 > 2 {
            let mid = c0 + (c1 - c0) / 2;
            if target < matrix[mid][0] {
                c1 = mid;
            } else if matrix[mid][0] < target {
                c0 = mid;
            } else {
                return true;
            }
        }
        println!("{},{}", c0, c1);

        for i in (c0..=c1).rev() {
            if matrix[i][0] <= target && matrix[i].binary_search(&target).is_ok() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0074() {
        let input = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]];
        assert_eq!(true, Solution::search_matrix(input.clone(), 3));
        assert_eq!(false, Solution::search_matrix(input, 13));

        let input2 = vec![vec![2, 3, 5, 7], vec![10, 11, 16, 20]];
        assert_eq!(true, Solution::search_matrix(input2.clone(), 3));
        assert_eq!(true, Solution::search_matrix(input2.clone(), 20));
        assert_eq!(true, Solution::search_matrix(input2.clone(), 2));
        assert_eq!(true, Solution::search_matrix(input2.clone(), 7));
        assert_eq!(true, Solution::search_matrix(input2.clone(), 10));
        assert_eq!(false, Solution::search_matrix(input2.clone(), 1));
        assert_eq!(false, Solution::search_matrix(input2, 33));
        assert_eq!(false, Solution::search_matrix(Vec::new(), 0));
        assert_eq!(true, Solution::search_matrix(vec![vec![3]], 3));
        assert_eq!(false, Solution::search_matrix(vec![vec![3]], 1));
        assert_eq!(false, Solution::search_matrix(vec![Vec::new()], 1));
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                11
            )
        );
        assert_eq!(
            false,
            Solution::search_matrix(
                vec![
                    vec![-9, -8, -8],
                    vec![-5, -3, -2],
                    vec![0, 2, 2],
                    vec![4, 6, 8]
                ],
                15
            )
        );
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![
                    vec![-10, -10],
                    vec![-9, -9],
                    vec![-8, -6],
                    vec![-4, -2],
                    vec![0, 1],
                    vec![3, 3],
                    vec![5, 5],
                    vec![6, 8]
                ],
                0
            )
        );
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![
                    vec![-10, -8],
                    vec![-6, -5],
                    vec![-2, -2],
                    vec![-1, 0],
                    vec![3, 4],
                    vec![7, 7],
                    vec![8, 9],
                    vec![10, 10],
                    vec![11, 11],
                    vec![12, 14],
                    vec![15, 16],
                    vec![17, 19],
                    vec![20, 21],
                    vec![22, 22],
                    vec![25, 27],
                    vec![28, 30],
                    vec![32, 32],
                    vec![35, 36]
                ],
                16
            )
        );
    }
}
