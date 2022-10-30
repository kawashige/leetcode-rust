pub struct Solution {}

impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut acc = vec![0; candies_count.len()];
        acc[0] = candies_count[0] as usize;
        for i in 1..candies_count.len() {
            acc[i] = acc[i - 1] + candies_count[i] as usize;
        }

        queries
            .into_iter()
            .map(|q| {
                let s = if q[0] == 0 {
                    0
                } else {
                    (acc[q[0] as usize - 1] + q[2] as usize) / q[2] as usize - 1
                };
                let e = acc[q[0] as usize] - 1;
                (s..=e).contains(&(q[1] as usize))
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1744() {
        assert_eq!(
            Solution::can_eat(
                vec![
                    46, 5, 47, 48, 43, 34, 15, 26, 11, 25, 41, 47, 15, 25, 16, 50, 32, 42, 32, 21,
                    36, 34, 50, 45, 46, 15, 46, 38, 50, 12, 3, 26, 26, 16, 23, 1, 4, 48, 47, 32,
                    47, 16, 33, 23, 38, 2, 19, 50, 6, 19, 29, 3, 27, 12, 6, 22, 33, 28, 7, 10, 12,
                    8, 13, 24, 21, 38, 43, 26, 35, 18, 34, 3, 14, 48, 50, 34, 38, 4, 50, 26, 5, 35,
                    11, 2, 35, 9, 11, 31, 36, 20, 21, 37, 18, 34, 34, 10, 21, 8, 5
                ],
                vec![vec![85, 54, 42],]
            ),
            vec![true]
        );
        assert_eq!(
            Solution::can_eat(
                vec![7, 4, 5, 3, 8],
                vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1000000000]]
            ),
            vec![true, false, true]
        );
        assert_eq!(
            Solution::can_eat(
                vec![5, 2, 6, 4, 1],
                vec![
                    vec![3, 1, 2],
                    vec![4, 10, 3],
                    vec![3, 10, 100],
                    vec![4, 100, 30],
                    vec![1, 3, 1]
                ]
            ),
            vec![false, true, true, false, false]
        );
    }
}
