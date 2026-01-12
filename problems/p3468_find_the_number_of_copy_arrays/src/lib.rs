pub struct Solution {}

impl Solution {
    pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        let mut range = bounds[0].clone();

        for i in 1..bounds.len() {
            let d = original[i] - original[i - 1];
            range = vec![range[0] + d, range[1] + d];

            if range[1] < bounds[i][0] || bounds[i][1] < range[0] {
                return 0;
            }

            range = vec![range[0].max(bounds[i][0]), range[1].min(bounds[i][1])];
        }

        range[1] - range[0] + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3468() {
        assert_eq!(
            Solution::count_arrays(
                vec![1, 2, 3, 4],
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
            ),
            2
        );
        assert_eq!(
            Solution::count_arrays(
                vec![1, 2, 3, 4],
                vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7]]
            ),
            4
        );
        assert_eq!(
            Solution::count_arrays(
                vec![1, 2, 1, 2],
                vec![vec![1, 1], vec![2, 3], vec![3, 3], vec![2, 3]]
            ),
            0
        );
    }
}
