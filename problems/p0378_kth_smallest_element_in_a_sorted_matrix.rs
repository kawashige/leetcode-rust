pub struct Solution {}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut route: Vec<i32> = vec![-1; matrix.len()];

        let mut rest = k;
        let mut min = 0;
        while 0 < rest {
            min = std::i32::MAX;
            let mut min_i = 0;
            for i in 0..matrix.len() {
                if matrix[0].len() as i32 - 1 < route[i] + 1 {
                    continue;
                }
                if matrix[i][(route[i] + 1) as usize] < min {
                    min = matrix[i][(route[i] + 1) as usize];
                    min_i = i;
                }
                if route[i] == -1 {
                    break;
                }
            }
            route[min_i] += 1;
            rest -= 1;
        }

        min
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0378() {
        assert_eq!(
            13,
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8)
        );
        assert_eq!(9, Solution::kth_smallest(vec![vec![9]], 1));
    }
}
