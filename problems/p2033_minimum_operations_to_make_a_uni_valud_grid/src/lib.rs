pub struct Solution {}

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut values = grid.into_iter().flatten().collect::<Vec<_>>();
        values.sort_unstable();
        let target = values[values.len() / 2];
        let mut count = 0;

        for i in 0..values.len() {
            if (values[i] - target).abs() % x != 0 {
                return -1;
            }
            count += (values[i] - target).abs() / x;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2033() {
        assert_eq!(
            Solution::min_operations(
                vec![
                    vec![454, 328, 160, 286, 664],
                    vec![496, 538, 748, 244, 286],
                    vec![34, 244, 454, 706, 790],
                    vec![496, 538, 832, 958, 328],
                    vec![370, 874, 370, 874, 286]
                ],
                42
            ),
            122
        );
        assert_eq!(Solution::min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
        assert_eq!(Solution::min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
        assert_eq!(
            Solution::min_operations(vec![vec![1, 2], vec![3, 4]], 2),
            -1
        );
    }
}
