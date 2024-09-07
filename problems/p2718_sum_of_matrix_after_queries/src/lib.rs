pub struct Solution {}

impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0;
        let mut counts = [n; 2];
        let mut seen = vec![vec![false; n as usize]; 2];

        for q in queries.into_iter().rev() {
            if seen[q[0] as usize][q[1] as usize] {
                continue;
            }
            seen[q[0] as usize][q[1] as usize] = true;
            sum += q[2] as i64 * counts[q[0] as usize] as i64;
            counts[(q[0] as usize + 1) % 2] -= 1;
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2718() {
        assert_eq!(
            Solution::matrix_sum_queries(
                3,
                vec![vec![0, 0, 1], vec![1, 2, 2], vec![0, 2, 3], vec![1, 0, 4]]
            ),
            23
        );
        assert_eq!(
            Solution::matrix_sum_queries(
                3,
                vec![
                    vec![0, 0, 4],
                    vec![0, 1, 2],
                    vec![1, 0, 1],
                    vec![0, 2, 3],
                    vec![1, 2, 1]
                ]
            ),
            17
        );
    }
}
