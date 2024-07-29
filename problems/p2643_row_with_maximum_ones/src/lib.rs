pub struct Solution {}

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0, 0];
        for i in 0..mat.len() {
            let count = mat[i].iter().sum::<i32>();
            if result[1] < count {
                result = vec![i as i32, count];
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2643() {
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]),
            vec![0, 1]
        );
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            vec![1, 2]
        );
    }
}
