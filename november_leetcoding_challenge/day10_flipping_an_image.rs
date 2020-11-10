pub struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; a[0].len()]; a.len()];
        for i in 0..a.len() {
            for j in 0..a[0].len() {
                if a[i][j] == 0 {
                    result[i][a[0].len() - 1 - j] = 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day10() {
        assert_eq!(
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]])
        );
        assert_eq!(
            vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ],
            Solution::flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ])
        );
    }
}
