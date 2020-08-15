pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push(vec![0; n as usize]);
        }

        let moves: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut m = 0;
        let mut p: (i32, i32) = (0, 0);
        for i in 1..=(n * n) {
            result[p.0 as usize][p.1 as usize] = i;
            if p.0 + moves[m].0 >= n
                || p.1 + moves[m].1 >= n
                || p.0 + moves[m].0 < 0
                || p.1 + moves[m].1 < 0
                || result[(p.0 + moves[m].0) as usize][(p.1 + moves[m].1) as usize] != 0
            {
                m = (m + 1) % 4;
            }
            p = (p.0 + moves[m].0, p.1 + moves[m].1);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0059() {
        let result = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(result, Solution::generate_matrix(3));

        let result2 = vec![
            vec![1, 2, 3, 4],
            vec![12, 13, 14, 5],
            vec![11, 16, 15, 6],
            vec![10, 9, 8, 7],
        ];
        assert_eq!(result2, Solution::generate_matrix(4));

        let result3 = vec![vec![1, 2], vec![4, 3]];
        assert_eq!(result3, Solution::generate_matrix(2));
        assert_eq!(vec![vec![1]], Solution::generate_matrix(1));
    }
}
