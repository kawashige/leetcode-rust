pub struct Solution {}

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut left = vec![[arrays[0][0], arrays[0][arrays[0].len() - 1]]; arrays.len()];
        let mut right = vec![
            [
                arrays[arrays.len() - 1][0],
                arrays[arrays.len() - 1][arrays[arrays.len() - 1].len() - 1]
            ];
            arrays.len()
        ];

        for i in 1..arrays.len() {
            left[i][0] = arrays[i][0].min(left[i - 1][0]);
            left[i][1] = arrays[i][arrays[i].len() - 1].max(left[i - 1][1]);
        }
        for i in (0..arrays.len() - 1).rev() {
            right[i][0] = arrays[i][0].min(right[i + 1][0]);
            right[i][1] = arrays[i][arrays[i].len() - 1].max(right[i + 1][1]);
        }

        let mut result = 0;
        for i in 0..arrays.len() {
            if 0 < i {
                result = result
                    .max((arrays[i][arrays[i].len() - 1] - left[i - 1][0]).abs())
                    .max((arrays[i][0] - left[i - 1][1]).abs())
            }
            if i < arrays.len() - 1 {
                result = result
                    .max((arrays[i][arrays[i].len() - 1] - right[i + 1][0]).abs())
                    .max((arrays[i][0] - right[i + 1][1]).abs())
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0624() {
        assert_eq!(Solution::max_distance(vec![vec![0, 2, 5], vec![4, 9]]), 4);
        assert_eq!(Solution::max_distance(vec![vec![1, 4], vec![0, 5]]), 4);
        assert_eq!(
            Solution::max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]),
            4
        );
        assert_eq!(Solution::max_distance(vec![vec![1], vec![1]]), 0);
    }
}
