pub struct Solution {}

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        ops.into_iter()
            .fold(vec![m, n], |acc, op| {
                vec![std::cmp::min(acc[0], op[0]), std::cmp::min(acc[1], op[1])]
            })
            .into_iter()
            .product()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0598() {
        assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
        assert_eq!(
            Solution::max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3]
                ]
            ),
            4
        );
        assert_eq!(Solution::max_count(3, 3, vec![]), 9);
        assert_eq!(Solution::max_count(1, 1, vec![vec![1, 1]]), 1);
    }
}
