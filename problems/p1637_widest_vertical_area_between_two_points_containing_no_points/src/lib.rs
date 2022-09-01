pub struct Solution {}

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|point| point[0]);
        points
            .windows(2)
            .into_iter()
            .map(|p| p[1][0] - p[0][0])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1637() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![8, 7],
                vec![9, 9],
                vec![7, 4],
                vec![9, 7]
            ]),
            1
        );
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8]
            ]),
            3
        );
    }
}
