pub struct Solution {}

impl Solution {
    pub fn check_straight_line(mut coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() == 2 {
            true
        } else {
            coordinates.sort_unstable();
            (2..coordinates.len()).all(|i| {
                (coordinates[1][1] - coordinates[0][1]) * (coordinates[i][0] - coordinates[0][0])
                    == (coordinates[i][1] - coordinates[0][1])
                        * (coordinates[1][0] - coordinates[0][0])
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1232() {
        assert!(Solution::check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ]));
        assert!(!Solution::check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]));
    }
}
