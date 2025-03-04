pub struct Solution {}

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions
            .into_iter()
            .map(|d| (d[0] * d[0] + d[1] * d[1], d[0] * d[1]))
            .max()
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3000() {
        assert_eq!(
            Solution::area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]]),
            48
        );
        assert_eq!(
            Solution::area_of_max_diagonal(vec![vec![3, 4], vec![4, 3]]),
            12
        );
    }
}
